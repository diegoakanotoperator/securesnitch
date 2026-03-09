use crate::protocol::ui_client::UiClient;
use tonic::transport::Channel;

pub mod protocol {
    tonic::include_proto!("protocol");
}

pub async fn create_client(addr: &str) -> anyhow::Result<UiClient<Channel>> {
    let channel = Channel::from_shared(addr.to_string())?
        .connect()
        .await?;
    
    Ok(UiClient::new(channel))
}
