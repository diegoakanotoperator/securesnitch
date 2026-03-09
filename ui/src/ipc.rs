use tonic::transport::{Channel, ClientTlsConfig, Identity, Certificate};

pub async fn connect_daemon_mtls(
    addr: &str,
    client_cert: &[u8],
    client_key: &[u8],
    ca_cert: &[u8]
) -> anyhow::Result<Channel> {
    let client_identity = Identity::from_pem(client_cert, client_key);
    let server_ca_cert = Certificate::from_pem(ca_cert);

    let tls_config = ClientTlsConfig::new()
        .domain_name("localhost")
        .identity(client_identity)
        .ca_certificate(server_ca_cert);

    let channel = Channel::from_shared(addr.to_string())?
        .tls_config(tls_config)?
        .connect()
        .await?;

    Ok(channel)
}