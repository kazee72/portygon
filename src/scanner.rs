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

    // Attempt TCP connection with timeout
    match timeout(Duration::from_secs(3), TcpStream::connect(socket)).await {
        Ok(Ok(mut stream)) => {
            let mut buf = vec![0u8; 1024];
            // Send HTTP request if port is a known HTTP port
            if http_ports.contains(&port) {
                let request = format!("GET / HTTP/1.1\r\nHost: {}\r\n\r\n", ip_str);
                stream.write_all(request.as_bytes()).await.ok();
            }

            if let Ok(Ok(bytes_read)) = timeout(Duration::from_secs(2), stream.read(&mut buf)).await {
                let banner = String::from_utf8_lossy(&buf[..bytes_read]).to_string();
                // Extract status line and Server header from HTTP response
                if banner.starts_with("HTTP") {
                    Some(parse_banner(&banner))
                } else {
                    Some(banner)
                }
            // Port is open but no banner received
            } else {
                Some(String::new())
            }    
        }
        // Connection failed or timed out
        _ => {
            None
        }
    }
}



/// Extracts the status line and Server header from a raw HTTP response.
///
/// # Arguments
/// * `raw_banner` - Raw HTTP response string
///
/// # Returns
/// A formatted string with the status line and Server header separated by " | "
pub fn parse_banner(raw_banner: &str) -> String {
    let mut http_banner = String::new();
    let split_banner: Vec<&str> = raw_banner.split("\r\n").collect();
    http_banner += split_banner[0];

    for line in split_banner {
        if line.starts_with("Server:") {
            http_banner += " | ";
            http_banner += line;
        }
    }
    http_banner
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_http_response() {
        let raw = "HTTP/1.1 200 OK\r\nDate: Wed, 01 Apr 2026\r\nServer: Apache/2.4.7 (Ubuntu)\r\nContent-Type: text/html\r\n\r\n<html>...</html>";
        let expected = "HTTP/1.1 200 OK | Server: Apache/2.4.7 (Ubuntu)";
        let output = parse_banner(raw);

        assert_eq!(expected, output);
    }

    #[test]
    fn test_response_without_server_header() {
        let raw = "HTTP/1.1 200 OK\r\nDate: Wed, 01 Apr 2026\r\nContent-Type: text/html\r\n\r\n<html>...</html>";
        let expected = "HTTP/1.1 200 OK";
        let output = parse_banner(raw);

        assert_eq!(expected, output);
    }

    #[test]
    fn test_status_line_only() {
        let raw = "HTTP/1.1 200 OK";
        let expected = "HTTP/1.1 200 OK";
        let output = parse_banner(raw);

        assert_eq!(expected, output);
    }

    #[test]
    fn test_empty_string() {
        let raw = "";
        let expected = "";
        let output = parse_banner(raw);

        assert_eq!(expected, output);
    }

    #[test]
    fn test_server_with_unusual_value() {
        let raw = "HTTP/1.1 200 OK\r\nServer: nginx/1.18.0 (Ubuntu)\r\nContent-Type: text/html\r\n\r\n";
        let expected = "HTTP/1.1 200 OK | Server: nginx/1.18.0 (Ubuntu)";
        let output = parse_banner(raw);

        assert_eq!(expected, output);
    }

    #[test]
    fn test_servername_should_not_match() {
        let raw = "HTTP/1.1 200 OK\r\nServerName: myhost.local\r\nServer: Apache/2.4.7\r\n\r\n";
        let expected = "HTTP/1.1 200 OK | Server: Apache/2.4.7";
        let output = parse_banner(raw);

        assert_eq!(expected, output);
    }
}