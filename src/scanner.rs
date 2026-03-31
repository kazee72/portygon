use std::{net::{IpAddr, SocketAddr}, time::Duration};
use tokio::time::timeout;
use tokio::net::TcpStream;



pub async fn scan(ip_str: &str, port: u16) -> bool {
    let ip: IpAddr = ip_str.parse().unwrap();
    let socket = SocketAddr::new(ip, port);

    matches!(timeout(Duration::from_secs(3), TcpStream::connect(socket)).await, Ok(Ok(_)))
}