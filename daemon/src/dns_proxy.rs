use tokio::net::UdpSocket;
use trust_dns_proto::op::Message;
use trust_dns_proto::serialize::binary::BinDecodable;
use reqwest::Client;

pub async fn start_dns_proxy(listen_addr: &str) -> anyhow::Result<()> {
    let socket = UdpSocket::bind(listen_addr).await?;
    let _client = Client::new();
    let _doh_url = "https://cloudflare-dns.com/dns-query";

    println!("DNS Proxy listening on {}", listen_addr);

    let mut buf = [0u16; 512];
    loop {
        let (len, src) = socket.recv_from(unsafe { std::mem::transmute::<&mut [u16; 512], &mut [u8; 1024]>(&mut buf) }).await?;
        let _query = Message::from_bytes(&unsafe { std::mem::transmute::<[u16; 512], [u8; 1024]>(buf) }[..len])?;
        
        println!("Intercepted DNS query from {}", src);
    }
}