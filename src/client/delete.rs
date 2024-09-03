use grammers_client::{client::bots::InvocationError, Client};
use grammers_tl_types::{self as tl, enums, types};
use tracing::instrument;

#[instrument(skip_all, fields(id, access_hash))]
pub async fn delete_channel(
    client: &Client,
    id: i64,
    access_hash: Option<i64>,
) -> Result<(), InvocationError> {
    client
        .invoke(&tl::functions::channels::DeleteChannel {
            channel: enums::InputChannel::Channel(types::InputChannel {
                channel_id: id,
                access_hash: access_hash.unwrap_or(0),
            }),
        })
        .await?;

    Ok(())
}
