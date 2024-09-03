use grammers_client::types::Message;
use grammers_session::PackedChat;
use grammers_tl_types::types as tl_types;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct MessageDisplay(pub Box<Message>);

impl From<Message> for MessageDisplay {
    fn from(val: Message) -> Self {
        MessageDisplay(val.into())
    }
}

impl Display for MessageDisplay {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Msg(id={message_id}, date={date}, action={action:#?})",
            message_id = self.0.id(),
            date = self.0.date(),
            action = self.0.action()
        )
    }
}

#[derive(Debug)]
pub struct MessagesDisplay(pub Box<[MessageDisplay]>);

impl From<Box<[MessageDisplay]>> for MessagesDisplay {
    fn from(val: Box<[MessageDisplay]>) -> Self {
        MessagesDisplay(val)
    }
}

impl From<Vec<MessageDisplay>> for MessagesDisplay {
    fn from(val: Vec<MessageDisplay>) -> Self {
        val.into_boxed_slice().into()
    }
}

impl Display for MessagesDisplay {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let length = self.0.len();
        for (index, message) in self.0.iter().enumerate() {
            if index + 1 == length {
                write!(f, "{message}")?;
            } else {
                write!(f, "{message}, ")?;
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct ChatLastMessageOld {
    pub packed: PackedChat,
    pub name: Box<str>,
    pub username: Option<Box<str>>,
    pub message: MessageDisplay,
}

impl Display for ChatLastMessageOld {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(ref username) = self.username {
            write!(
                f,
                "{ty}({chat_id}, @{username}, title={name}, access_hash={access_hash})\n{message}",
                ty = self.packed.ty,
                chat_id = self.packed.id,
                name = self.name,
                access_hash = self
                    .packed
                    .access_hash
                    .map_or("unknown".to_owned(), |access_hash| access_hash.to_string()),
                message = self.message,
            )
        } else {
            write!(
                f,
                "{ty}({chat_id}, title={name}, access_hash={access_hash})\n{message}",
                ty = self.packed.ty,
                chat_id = self.packed.id,
                name = self.name,
                access_hash = self
                    .packed
                    .access_hash
                    .map_or("unknown".to_owned(), |access_hash| access_hash.to_string()),
                message = self.message
            )
        }
    }
}

#[derive(Debug)]
pub struct LeavedMessageOld {
    pub chat: tl_types::Chat,
    pub message: MessageDisplay,
}

impl Display for LeavedMessageOld {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Chat({chat_id}, title={title})\n{message}",
            chat_id = self.chat.id,
            title = self.chat.title,
            message = self.message,
        )
    }
}

#[derive(Debug)]
pub struct LeavedChannelMessageOld {
    pub channel: tl_types::Channel,
    pub message: MessageDisplay,
}

impl Display for LeavedChannelMessageOld {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(ref username) = self.channel.username {
            write!(
                f,
                "Channel({chat_id}, @{username}, title={title}, access_hash={access_hash})\n{message}",
                chat_id = self.channel.id,
                title = self.channel.title,
                access_hash = self
                    .channel
                    .access_hash
                    .map_or("unknown".to_owned(), |access_hash| access_hash.to_string()),
                message = self.message,
            )
        } else {
            write!(
                f,
                "Channel({chat_id}, title={title}, access_hash={access_hash})\n{message}",
                chat_id = self.channel.id,
                title = self.channel.title,
                access_hash = self
                    .channel
                    .access_hash
                    .map_or("unknown".to_owned(), |access_hash| access_hash.to_string()),
                message = self.message,
            )
        }
    }
}

#[derive(Debug)]
pub struct LastMessagesOld {
    pub packed: PackedChat,
    pub name: Box<str>,
    pub username: Option<Box<str>>,
    pub messages: MessagesDisplay,
}

impl Display for LastMessagesOld {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(ref username) = self.username {
            write!(
                f,
                "{ty}({chat_id}, @{username}, title={name}, access_hash={access_hash})\n{messages}",
                ty = self.packed.ty,
                chat_id = self.packed.id,
                name = self.name,
                access_hash = self
                    .packed
                    .access_hash
                    .map_or("unknown".to_owned(), |access_hash| access_hash.to_string()),
                messages = self.messages
            )
        } else {
            write!(
                f,
                "{ty}({chat_id}, title={name}, access_hash={access_hash})\n{messages}",
                ty = self.packed.ty,
                chat_id = self.packed.id,
                name = self.name,
                access_hash = self
                    .packed
                    .access_hash
                    .map_or("unknown".to_owned(), |access_hash| access_hash.to_string()),
                messages = self.messages
            )
        }
    }
}

#[derive(Debug)]
pub struct LeavedMessagesOld {
    pub chat: tl_types::Chat,
    pub messages: MessagesDisplay,
}

impl Display for LeavedMessagesOld {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Chat({chat_id}, title={title})\n{messages}",
            chat_id = self.chat.id,
            title = self.chat.title,
            messages = self.messages,
        )
    }
}

#[derive(Debug)]
pub struct LeavedChannelMessagesOld {
    pub channel: tl_types::Channel,
    pub messages: MessagesDisplay,
}

impl Display for LeavedChannelMessagesOld {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(ref username) = self.channel.username {
            write!(
                f,
                "Channel({chat_id}, @{username}, title={title}, access_hash={access_hash})\n{messages}",
                chat_id = self.channel.id,
                title = self.channel.title,
                access_hash = self
                    .channel
                    .access_hash
                    .map_or("unknown".to_owned(), |access_hash| access_hash.to_string()),
                messages = self.messages,
            )
        } else {
            write!(
                f,
                "Channel({chat_id}, title={title}, access_hash={access_hash})\n{messages}",
                chat_id = self.channel.id,
                title = self.channel.title,
                access_hash = self
                    .channel
                    .access_hash
                    .map_or("unknown".to_owned(), |access_hash| access_hash.to_string()),
                messages = self.messages,
            )
        }
    }
}

#[derive(Debug)]
pub struct MessagesEmpty {
    pub packed: PackedChat,
    pub name: Box<str>,
    pub username: Option<Box<str>>,
}

impl Display for MessagesEmpty {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(ref username) = self.username {
            write!(
                f,
                "{ty}({chat_id}, @{username}, title={name}, access_hash={access_hash})",
                ty = self.packed.ty,
                chat_id = self.packed.id,
                name = self.name,
                access_hash = self
                    .packed
                    .access_hash
                    .map_or("unknown".to_owned(), |access_hash| access_hash.to_string()),
            )
        } else {
            write!(
                f,
                "{ty}({chat_id}, title={name}, access_hash={access_hash})",
                ty = self.packed.ty,
                chat_id = self.packed.id,
                name = self.name,
                access_hash = self
                    .packed
                    .access_hash
                    .map_or("unknown".to_owned(), |access_hash| access_hash.to_string()),
            )
        }
    }
}

#[derive(Debug)]
pub struct Empty {
    pub id: i64,
}

impl Display for Empty {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Empty({chat_id})", chat_id = self.id)
    }
}

#[derive(Debug)]
pub struct MessagesCountSmall {
    pub packed: PackedChat,
    pub name: Box<str>,
    pub username: Option<Box<str>>,
    pub messages: MessagesDisplay,
}

impl Display for MessagesCountSmall {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(ref username) = self.username {
            write!(
                f,
                "{ty}({chat_id}, @{username}, title={name}, access_hash={access_hash})\n{messages}",
                ty = self.packed.ty,
                chat_id = self.packed.id,
                name = self.name,
                access_hash = self
                    .packed
                    .access_hash
                    .map_or("unknown".to_owned(), |access_hash| access_hash.to_string()),
                messages = self.messages,
            )
        } else {
            write!(
                f,
                "{ty}({chat_id}, title={name}, access_hash={access_hash})\n{messages}",
                ty = self.packed.ty,
                chat_id = self.packed.id,
                name = self.name,
                access_hash = self
                    .packed
                    .access_hash
                    .map_or("unknown".to_owned(), |access_hash| access_hash.to_string()),
                messages = self.messages,
            )
        }
    }
}

#[derive(Debug)]
pub struct LeavedMessagesCountSmall {
    pub chat: tl_types::Chat,
    pub messages: MessagesDisplay,
}

impl Display for LeavedMessagesCountSmall {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Chat({chat_id}, title={title})\n{messages}",
            chat_id = self.chat.id,
            title = self.chat.title,
            messages = self.messages,
        )
    }
}

#[derive(Debug)]
pub struct LeavedChannelMessagesCountSmall {
    pub channel: tl_types::Channel,
    pub messages: MessagesDisplay,
}

impl Display for LeavedChannelMessagesCountSmall {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(ref username) = self.channel.username {
            write!(
                f,
                "Channel({chat_id}, @{username}, title={title}, access_hash={access_hash})\n{messages}",
                chat_id = self.channel.id,
                messages = self.messages,
                access_hash = self
                    .channel
                    .access_hash
                    .map_or("unknown".to_owned(), |access_hash| access_hash.to_string()),
                title = self.channel.title,
            )
        } else {
            write!(
                f,
                "Channel({chat_id}, title={title}, access_hash={access_hash})\n{messages}",
                chat_id = self.channel.id,
                messages = self.messages,
                access_hash = self
                    .channel
                    .access_hash
                    .map_or("unknown".to_owned(), |access_hash| access_hash.to_string()),
                title = self.channel.title,
            )
        }
    }
}

#[derive(Debug)]
pub struct CreatorLeaved {
    pub chat: tl_types::Chat,
}

impl Display for CreatorLeaved {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Chat({chat_id}, title={title})",
            chat_id = self.chat.id,
            title = self.chat.title,
        )
    }
}

#[derive(Debug)]
pub struct ChannelCreatorLeaved {
    pub channel: Box<tl_types::Channel>,
}

impl Display for ChannelCreatorLeaved {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(ref username) = self.channel.username {
            write!(
                f,
                "Channel({chat_id}, @{username}, title={title}, access_hash={access_hash})",
                chat_id = self.channel.id,
                title = self.channel.title,
                access_hash = self
                    .channel
                    .access_hash
                    .map_or("unknown".to_owned(), |access_hash| access_hash.to_string()),
            )
        } else {
            write!(
                f,
                "Channel({chat_id}, title={title}, access_hash={access_hash})",
                chat_id = self.channel.id,
                title = self.channel.title,
                access_hash = self
                    .channel
                    .access_hash
                    .map_or("unknown".to_owned(), |access_hash| access_hash.to_string())
            )
        }
    }
}

#[derive(Debug)]
pub enum Chat {
    LastMessageOld(ChatLastMessageOld),
    LeavedMessageOld(LeavedMessageOld),
    LeavedChannelMessageOld(LeavedChannelMessageOld),
    LastMessagesOld(LastMessagesOld),
    LeavedMessagesOld(LeavedMessagesOld),
    LeavedChannelMessagesOld(LeavedChannelMessagesOld),
    MessagesEmpty(MessagesEmpty),
    Empty(Empty),
    MessagesCountSmall(MessagesCountSmall),
    LeavedMessagesCountSmall(LeavedMessagesCountSmall),
    LeavedChannelMessagesCountSmall(LeavedChannelMessagesCountSmall),
    CreatorLeaved(CreatorLeaved),
    ChannelCreatorLeaved(ChannelCreatorLeaved),
}

impl Chat {
    pub const fn id(&self) -> i64 {
        match self {
            Chat::LastMessageOld(ChatLastMessageOld {
                packed: PackedChat { id, .. },
                ..
            })
            | Chat::MessagesEmpty(MessagesEmpty {
                packed: PackedChat { id, .. },
                ..
            })
            | Chat::LastMessagesOld(LastMessagesOld {
                packed: PackedChat { id, .. },
                ..
            })
            | Chat::Empty(Empty { id })
            | Chat::MessagesCountSmall(MessagesCountSmall {
                packed: PackedChat { id, .. },
                ..
            }) => *id,
            Chat::LeavedMessageOld(LeavedMessageOld { chat, .. })
            | Chat::LeavedMessagesOld(LeavedMessagesOld { chat, .. })
            | Chat::LeavedMessagesCountSmall(LeavedMessagesCountSmall { chat, .. })
            | Chat::CreatorLeaved(CreatorLeaved { chat }) => chat.id,
            Chat::LeavedChannelMessageOld(LeavedChannelMessageOld { channel, .. })
            | Chat::LeavedChannelMessagesOld(LeavedChannelMessagesOld { channel, .. })
            | Chat::LeavedChannelMessagesCountSmall(LeavedChannelMessagesCountSmall {
                channel,
                ..
            }) => channel.id,
            Chat::ChannelCreatorLeaved(ChannelCreatorLeaved { channel, .. }) => channel.id,
        }
    }

    pub const fn access_hash(&self) -> Option<i64> {
        match self {
            Chat::LastMessageOld(ChatLastMessageOld {
                packed: PackedChat { access_hash, .. },
                ..
            })
            | Chat::LastMessagesOld(LastMessagesOld {
                packed: PackedChat { access_hash, .. },
                ..
            })
            | Chat::MessagesEmpty(MessagesEmpty {
                packed: PackedChat { access_hash, .. },
                ..
            })
            | Chat::MessagesCountSmall(MessagesCountSmall {
                packed: PackedChat { access_hash, .. },
                ..
            }) => *access_hash,
            Chat::LeavedChannelMessageOld(LeavedChannelMessageOld { channel, .. })
            | Chat::LeavedChannelMessagesOld(LeavedChannelMessagesOld { channel, .. })
            | Chat::LeavedChannelMessagesCountSmall(LeavedChannelMessagesCountSmall {
                channel,
                ..
            }) => channel.access_hash,
            Chat::ChannelCreatorLeaved(ChannelCreatorLeaved { channel, .. }) => channel.access_hash,
            Chat::LeavedMessageOld(_)
            | Chat::LeavedMessagesOld(_)
            | Chat::Empty(_)
            | Chat::LeavedMessagesCountSmall(_)
            | Chat::CreatorLeaved(_) => None,
        }
    }
}

impl Display for Chat {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Chat::LastMessageOld(val) => write!(f, "Last message too old: {val}"),
            Chat::LeavedMessageOld(val) => write!(f, "Last message too old: {val}"),
            Chat::LeavedChannelMessageOld(val) => write!(f, "Last message too old: {val}"),
            Chat::LastMessagesOld(val) => write!(f, "Last messages too old: {val}"),
            Chat::LeavedMessagesOld(val) => write!(f, "Last messages too old: {val}"),
            Chat::LeavedChannelMessagesOld(val) => write!(f, "Last messages too old: {val}"),
            Chat::MessagesEmpty(val) => write!(f, "Messages empty: {val}"),
            Chat::Empty(val) => write!(f, "{val}"),
            Chat::MessagesCountSmall(val) => write!(f, "Messages count too small: {val}"),
            Chat::LeavedMessagesCountSmall(val) => write!(f, "Messages count too small: {val}"),
            Chat::LeavedChannelMessagesCountSmall(val) => {
                write!(f, "Messages count too small: {val}")
            }
            Chat::CreatorLeaved(val) => write!(f, "Leaved as creator: {val}"),
            Chat::ChannelCreatorLeaved(val) => write!(f, "Leaved as creator: {val}"),
        }
    }
}
