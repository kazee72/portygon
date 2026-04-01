use std::{collections::HashSet, net::{IpAddr, SocketAddr}, time::Duration};
use tokio::time::timeout;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};


/// Scans a single port on the target host and attempts to grab the service banner.
///
/// # Arguments
/// * `ip_str` - Target IP address as a string
/// * `port` - Port number to scan
/// * `http_ports` - Set of ports that require an HTTP request for banner grabbing
///
/// # Returns
/// * `Some(banner)` - Port is open, with banner string (may be empty if no banner received)
/// * `None` - Port is closed or filtered
pub async fn scan(ip_str: &str, port: u16, http_ports: &HashSet<u16>) -> Option<String> {
    let ip: IpAddr = ip_str.parse().unwrap();
    let socket = SocketAddr::new(ip, port);

    match timeout(Duration::from_secs(3), TcpStream::connect(socket)).await {
        Ok(Ok(mut stream)) => {
            let mut buf = vec![0u8; 1024];
            if http_ports.contains(&port) {
                let request = format!("GET / HTTP/1.1\r\nHost: {}\r\n\r\n", ip_str);
                stream.write_all(request.as_bytes()).await.ok();
            }

            if let Ok(Ok(bytes_read)) = timeout(Duration::from_secs(2), stream.read(&mut buf)).await {
                let banner = String::from_utf8_lossy(&buf[..bytes_read]).to_string();
                if banner.starts_with("HTTP") {
                    let mut http_banner = String::new();
                    let split_banner: Vec<&str> = banner.split("\r\n").collect();
                    http_banner += split_banner[0];
                    for line in split_banner {
                        if line.starts_with("Server") {
                            http_banner += " | ";
                            http_banner += line;
                        }
                    }
                    Some(http_banner)
                } else {
                    Some(banner)
                }
            } else {
                Some(String::new())
            }    
        }
        _ => {
            None
        }
    }
}