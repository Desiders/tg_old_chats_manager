use grammers_client::{client::bots::InvocationError, Client};
use grammers_mtsender::RpcError;
use grammers_tl_types::{self as tl, enums, types};
use tracing::{debug, instrument};

#[instrument(skip_all, fields(id, access_hash))]
pub async fn get_chat_invite(
    client: &Client,
    id: i64,
    access_hash: Option<i64>,
) -> Result<Option<types::ChatInviteExported>, InvocationError> {
    match client
        .invoke(&tl::functions::messages::ExportChatInvite {
            legacy_revoke_permanent: false,
            request_needed: false,
            peer: enums::InputPeer::Channel(types::InputPeerChannel {
                channel_id: id,
                access_hash: access_hash.unwrap_or(0),
            }),
            expire_date: None,
            usage_limit: None,
            title: None,
            subscription_pricing: None,
        })
        .await
    {
        Ok(invite) => match invite {
            enums::ExportedChatInvite::ChatInviteExported(invite) => Ok(Some(invite)),
            enums::ExportedChatInvite::ChatInvitePublicJoinRequests => Ok(None),
        },
        Err(err) => match err {
            InvocationError::Rpc(RpcError {
                code: _code @ 403, ..
            }) => {
                debug!("Chat forbidden");

                Ok(None)
            }
            err => Err(err),
        },
    }
}