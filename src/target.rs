use std::net::IpAddr;

/// Resolves a target string into an IP address.
///
/// Accepts either a raw IP address or a hostname. Raw IPs are returned
/// directly; hostnames are resolved via DNS lookup. When a hostname
/// resolves to multiple addresses, the first one is returned.
///
/// # Arguments
/// * `target_string` - Target IP address or hostname
///
/// # Returns
/// * `Ok(IpAddr)` - The resolved IP address
/// * `Err(String)` - Error message if the target cannot be resolved
pub async fn resolve_target(target_string: &str) -> Result<IpAddr, String> {
    if let Ok(ip) = target_string.parse::<IpAddr>() {
        return Ok(ip);
    }

    let query = format!("{}:0", target_string);
    let mut addrs = match tokio::net::lookup_host(query).await {
        Ok(addrs) => addrs,
        Err(e) => return Err(format!("could not resolve '{}': {}", target_string, e)),
    };

    let socket_addr = addrs
        .next()
        .ok_or_else(|| format!("no addresses found for '{}'", target_string))?;

    Ok(socket_addr.ip())
}
