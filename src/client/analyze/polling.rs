use chrono::{NaiveTime, Utc};
use grammers_client::{
    client::bots::InvocationError,
    types::{self, Dialog, Message},
    Client,
};
use grammers_mtsender::RpcError;
use grammers_session::{PackedChat, PackedType};
use grammers_tl_types::{self as tl, enums, types as tl_types};
use tracing::{debug, error, instrument, span, trace, Level};

use super::models::{
    ChannelCreatorLeaved, Chat, ChatLastMessageOld, CreatorLeaved, Empty, LastMessagesOld,
    LeavedChannelMessageOld, LeavedChannelMessagesCountSmall, LeavedChannelMessagesOld,
    LeavedMessageOld, LeavedMessagesCountSmall, LeavedMessagesOld, MessagesCountSmall,
    MessagesEmpty,
};

const OLD_MESSAGE_ELAPSED_DAYS: i64 = 30;
const ELAPSED_DAYS_BETWEEN_OLD_MESSAGES: i64 = OLD_MESSAGE_ELAPSED_DAYS;
const CHANNEL_ELAPSED_MULTIPLIER: i64 = 2;

const LAST_MESSAGES_LIMIT: usize = 15;
const OLD_MESSAGES_COUNT: usize = LAST_MESSAGES_LIMIT / 3;

const fn messages_count_is_too_small(messages_count: usize) -> bool {
    messages_count < 2
}

fn last_message_is_old(
    message: &Message,
    now_time: NaiveTime,
    old_message_elapsed_days: i64,
) -> bool {
    let last_message_time = message.date().time();
    let elapsed = now_time - last_message_time;

    elapsed.num_days() > old_message_elapsed_days
}

fn old_messages_count_limit_reached(
    messages: &[Message],
    elapsed_days_between_old_messages: i64,
) -> bool {
    let mut rev_messages = messages.to_vec();
    rev_messages.reverse();

    let mut old_messages_count = 0;
    let mut last_message = rev_messages.remove(0);
    let mut last_message_time = last_message.date().time();
    for message in rev_messages {
        let message_time = message.date().time();
        let elapsed = last_message_time - message_time;

        if elapsed.num_days() >= elapsed_days_between_old_messages {
            old_messages_count += 1;
        }

        last_message = message;
        last_message_time = message_time;
    }
    drop(last_message);

    old_messages_count >= OLD_MESSAGES_COUNT
}

#[instrument(skip_all)]
pub async fn get_chats(client: &Client) -> Result<Vec<Chat>, InvocationError> {
    let mut chats = vec![];
    let mut dialogs = client.iter_dialogs();
    let mut dialogs_count = 0;

    let now_time = Utc::now().time();

    while let Some(Dialog {
        chat, last_message, ..
    }) = dialogs.next().await?
    {
        dialogs_count += 1;

        let chat_id = chat.id();

        let span = span!(Level::DEBUG, "iter", chat_id, num = dialogs_count);
        let _guard = span.enter();

        let mut old_message_elapsed_days = OLD_MESSAGE_ELAPSED_DAYS;
        let mut elapsed_days_between_old_messages = ELAPSED_DAYS_BETWEEN_OLD_MESSAGES;
        match chat {
            types::Chat::User(_) => continue,
            types::Chat::Group(_) => {}
            types::Chat::Channel(ref channel) => {
                match channel.pack().ty {
                    PackedType::Megagroup | PackedType::Broadcast | PackedType::Gigagroup => {
                        old_message_elapsed_days *= CHANNEL_ELAPSED_MULTIPLIER;
                        elapsed_days_between_old_messages *= CHANNEL_ELAPSED_MULTIPLIER;
                    }
                    _ => {}
                };
            }
        };

        if let Some(last_message) = last_message {
            if last_message_is_old(&last_message, now_time, old_message_elapsed_days) {
                debug!(parent: &span, "Found an old chat by last message");

                chats.push(Chat::LastMessageOld(ChatLastMessageOld {
                    username: chat.username().map(Into::into),
                    name: chat.name().into(),
                    packed: chat.into(),
                    message: last_message.into(),
                }));
                continue;
            }
        } else {
            debug!(parent: &span, "Last message not found");

            chats.push(Chat::MessagesEmpty(MessagesEmpty {
                username: chat.username().map(Into::into),
                name: chat.name().into(),
                packed: chat.into(),
            }));
            continue;
        };

        let mut messages_iter = client.iter_messages(&chat).limit(LAST_MESSAGES_LIMIT);
        let mut messages = Vec::with_capacity(LAST_MESSAGES_LIMIT);
        while let Some(message) = messages_iter.next().await? {
            messages.push(message);
        }

        if messages_count_is_too_small(messages.len()) {
            debug!(parent: &span, "Messages count in the chat is too small");

            chats.push(Chat::MessagesCountSmall(MessagesCountSmall {
                username: chat.username().map(Into::into),
                name: chat.name().into(),
                packed: chat.into(),
                messages: messages
                    .into_iter()
                    .map(Into::into)
                    .collect::<Vec<_>>()
                    .into(),
            }));
        } else if old_messages_count_limit_reached(&messages, elapsed_days_between_old_messages) {
            debug!(parent: &span, "Found an old chat by last messages which are periodically sent with high delay");

            chats.push(Chat::LastMessagesOld(LastMessagesOld {
                username: chat.username().map(Into::into),
                name: chat.name().into(),
                packed: chat.into(),
                messages: messages
                    .into_iter()
                    .map(Into::into)
                    .collect::<Vec<_>>()
                    .into(),
            }));
        }
    }

    Ok(chats)
}

#[instrument(skip_all)]
pub async fn init_takeout_session(client: &Client) -> Result<i64, InvocationError> {
    client
        .invoke(&tl::functions::account::InitTakeoutSession {
            contacts: false,
            message_users: false,
            message_chats: true,
            message_megagroups: true,
            message_channels: true,
            files: false,
            file_max_size: None,
        })
        .await
        .map(|enums::account::Takeout::Takeout(tl_types::account::Takeout { id })| id)
}

#[instrument(skip_all, fields(takeout_id, success))]
pub async fn finish_takeout_session(
    client: &Client,
    takeout_id: i64,
    success: bool,
) -> Result<bool, InvocationError> {
    client
        .invoke(&tl::functions::InvokeWithTakeout {
            takeout_id,
            query: tl::functions::account::FinishTakeoutSession { success },
        })
        .await
}

#[instrument(skip_all)]
pub async fn get_left_chats(
    client: &Client,
    takeout_id: i64,
) -> Result<Vec<Chat>, InvocationError> {
    let mut chats = vec![];
    let mut chats_count = 0;

    let now_time = Utc::now().time();

    let left_chats = match client
        .invoke(&tl::functions::InvokeWithTakeout {
            takeout_id,
            query: tl::functions::channels::GetLeftChannels { offset: 0 },
        })
        .await?
    {
        enums::messages::Chats::Chats(tl_types::messages::Chats { chats })
        | enums::messages::Chats::Slice(tl_types::messages::ChatsSlice { chats, .. }) => chats,
    };

    'outer: for chat in left_chats {
        chats_count += 1;

        let chat_id = chat.id();

        let span = span!(Level::DEBUG, "iter", chat_id, num = chats_count);
        let _guard = span.enter();

        match chat {
            enums::Chat::Empty(_) => {
                debug!(parent: &span, "Found an old chat by empty chat");

                chats.push(Chat::Empty(Empty { id: chat_id }));
                continue;
            }
            enums::Chat::Chat(chat) => {
                if chat.creator {
                    debug!(parent: &span, "Found an old chat in which you're the creator");

                    chats.push(Chat::CreatorLeaved(CreatorLeaved { chat }));
                    continue;
                }

                let mut messages_iter = client
                    .iter_messages(PackedChat {
                        ty: PackedType::Chat,
                        id: chat_id,
                        access_hash: None,
                    })
                    .limit(LAST_MESSAGES_LIMIT);
                let mut messages = Vec::with_capacity(LAST_MESSAGES_LIMIT);

                while let Some(message) = match messages_iter.next().await {
                    Ok(message) => message,
                    Err(err) => {
                        match err {
                            InvocationError::Rpc(RpcError {
                                code: _code @ 400, ..
                            }) => {
                                debug!(parent: &span, "Chat is private");
                            }
                            _ => {
                                error!(parent: &span, %err, "Error while get chat messages");
                            }
                        };

                        continue 'outer;
                    }
                } {
                    messages.push(message);
                }

                if messages_count_is_too_small(messages.len()) {
                    debug!(parent: &span, "Messages count in the leaved chat is too small");

                    chats.push(Chat::LeavedMessagesCountSmall(LeavedMessagesCountSmall {
                        chat,
                        messages: messages
                            .into_iter()
                            .map(Into::into)
                            .collect::<Vec<_>>()
                            .into(),
                    }));
                    continue;
                }

                let last_message = messages.remove(0);
                if last_message_is_old(
                    &last_message,
                    now_time,
                    OLD_MESSAGE_ELAPSED_DAYS * CHANNEL_ELAPSED_MULTIPLIER,
                ) {
                    debug!(parent: &span, "Found an old leaved chat by last message");

                    chats.push(Chat::LeavedMessageOld(LeavedMessageOld {
                        chat,
                        message: last_message.into(),
                    }));
                } else if old_messages_count_limit_reached(
                    &messages,
                    ELAPSED_DAYS_BETWEEN_OLD_MESSAGES * CHANNEL_ELAPSED_MULTIPLIER,
                ) {
                    debug!(parent: &span, "Found an old leaved chat by last messages which are periodically sent with high delay");

                    chats.push(Chat::LeavedMessagesOld(LeavedMessagesOld {
                        chat,
                        messages: messages
                            .into_iter()
                            .map(Into::into)
                            .collect::<Vec<_>>()
                            .into(),
                    }));
                }
            }
            enums::Chat::Channel(channel) => {
                if channel.creator {
                    debug!(parent: &span, "Found an old channel in which you're the creator");

                    chats.push(Chat::ChannelCreatorLeaved(ChannelCreatorLeaved {
                        channel: Box::new(channel),
                    }));
                    continue;
                }

                let ty = if channel.megagroup {
                    PackedType::Megagroup
                } else if channel.gigagroup {
                    PackedType::Gigagroup
                } else if channel.broadcast {
                    PackedType::Broadcast
                } else {
                    unreachable!("Found incorrect packed type")
                };

                let mut messages_iter = client
                    .iter_messages(PackedChat {
                        ty,
                        id: channel.id,
                        access_hash: channel.access_hash,
                    })
                    .limit(LAST_MESSAGES_LIMIT);
                let mut messages = Vec::with_capacity(LAST_MESSAGES_LIMIT);

                while let Some(message) = match messages_iter.next().await {
                    Ok(message) => message,
                    Err(err) => {
                        match err {
                            InvocationError::Rpc(RpcError {
                                code: _code @ 400, ..
                            }) => {
                                debug!(parent: &span, "Channel is private");
                            }
                            _ => {
                                error!(parent: &span, %err, "Error while get channel messages");
                            }
                        };

                        continue 'outer;
                    }
                } {
                    messages.push(message);
                }

                if messages_count_is_too_small(messages.len()) {
                    debug!(parent: &span, "Messages count in the leaved channel is too small");

                    chats.push(Chat::LeavedChannelMessagesCountSmall(
                        LeavedChannelMessagesCountSmall {
                            channel,
                            messages: messages
                                .into_iter()
                                .map(Into::into)
                                .collect::<Vec<_>>()
                                .into(),
                        },
                    ));
                    continue;
                }

                let last_message = messages.remove(0);
                if last_message_is_old(
                    &last_message,
                    now_time,
                    OLD_MESSAGE_ELAPSED_DAYS * CHANNEL_ELAPSED_MULTIPLIER,
                ) {
                    debug!(parent: &span, "Found an old leaved channel by last message");

                    chats.push(Chat::LeavedChannelMessageOld(LeavedChannelMessageOld {
                        channel,
                        message: last_message.into(),
                    }));
                } else if old_messages_count_limit_reached(
                    &messages,
                    ELAPSED_DAYS_BETWEEN_OLD_MESSAGES * CHANNEL_ELAPSED_MULTIPLIER,
                ) {
                    debug!(parent: &span, "Found an old leaved channel by last messages which are periodically sent with high delay");

                    chats.push(Chat::LeavedChannelMessagesOld(LeavedChannelMessagesOld {
                        channel,
                        messages: messages
                            .into_iter()
                            .map(Into::into)
                            .collect::<Vec<_>>()
                            .into(),
                    }));
                }
            }
            enums::Chat::Forbidden(_) => {
                trace!(parent: &span, "Chat forbidden");
            }
            enums::Chat::ChannelForbidden(_) => {
                trace!(parent: &span, "Channel forbidden");
            }
        };
    }

    Ok(chats)
}
