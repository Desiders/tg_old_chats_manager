use std::process;

use super::models::{Analyze, Delete, Join};
use crate::client::{
    analyze::{self, Chat},
    delete, invite_link, join,
};
use grammers_client::{client::bots::InvocationError, Client};
use grammers_mtsender::RpcError;
use tracing::{debug, error};

pub async fn analyze(config: Analyze, client: &Client) -> Result<(), InvocationError> {
    let mut chats: Vec<Chat> = vec![];

    if config.joined {
        println!("Analyze the chats you are a member of. It may take a few minutes.");

        chats = chats
            .into_iter()
            .chain(analyze::get_chats(client).await.unwrap())
            .collect();
    }
    if config.left {
        println!("Analyze the chats that you're left. It may take a few minutes.");

        let takeout_id = match analyze::init_takeout_session(client).await {
            Ok(takeout_id) => takeout_id,
            Err(err) => {
                match err {
                    InvocationError::Rpc(RpcError {
                        code: code @ 420,
                        value,
                        ..
                    }) => {
                        error!(?value, code, "Sorry, for security reasons, you will be able to begin downloading your data in %d seconds. We have notified all your devices about the export request to make sure it's authorized and to give you time to react if it's not.");
                    }
                    _ => {
                        error!(%err, "Error while get group messages");
                    }
                };

                process::exit(1);
            }
        };

        let success = match analyze::get_left_chats(client, takeout_id).await {
            Ok(left_chats) => {
                chats = chats.into_iter().chain(left_chats).collect();
                true
            }
            Err(err) => {
                error!(%err, "Error while get left chats");
                false
            }
        };
        analyze::finish_takeout_session(client, takeout_id, success).await?;
    }

    for chat in chats {
        if let Some(chat_invite) =
            match invite_link::get_chat_invite(client, chat.id(), chat.access_hash()).await {
                Ok(val) => val,
                Err(err) => {
                    debug!(%err, "Error while get invite link");
                    None
                }
            }
        {
            println!("{chat} ({link})", link = chat_invite.link);
        } else {
            println!("{chat}");
        }
    }

    Ok(())
}

pub async fn join_channel(config: Join, client: &Client) -> Result<(), InvocationError> {
    match join::join_channel(client, config.id, config.access_hash).await {
        Ok(()) => {
            println!("You have joined the channel/supergroup");
            Ok(())
        }
        Err(InvocationError::Rpc(RpcError {
            code: _code @ 400, ..
        })) => {
            println!(
                "Channel invalid. Probably specified incorrect channel/supergroup ID or access hash is missing."
            );
            Ok(())
        }
        Err(err) => Err(err),
    }
}

pub async fn delete_channel(config: Delete, client: &Client) -> Result<(), InvocationError> {
    match delete::delete_channel(client, config.id, config.access_hash).await {
        Ok(()) => {
            println!("You have deleted the channel/supergroup");
            Ok(())
        }
        Err(InvocationError::Rpc(RpcError {
            code: _code @ 400, ..
        })) => {
            println!(
                "Channel invalid. Probably specified incorrect channel/supergroup ID or access hash is missing."
            );
            Ok(())
        }
        Err(err) => Err(err),
    }
}
