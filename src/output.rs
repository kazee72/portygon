use colored::Colorize;
use serde::Serialize;



#[derive(Serialize)]
struct ScanResult {
    target: String,
    ports_scanned: usize,
    open_ports: Vec<OpenPort>
}

#[derive(Serialize)]
struct OpenPort {
    port: u16,
    banner: String
}



/// Displays the scan results in the terminal with color-coded output.
///
/// Open ports are shown in green with their banner if available.
/// Prints a message if no open ports were found.
///
/// # Arguments
/// * `result` - Slice of (port, banner) tuples where `Some` indicates an open port
pub fn display_results(result: &[(u16, Option<String>)]) {
    let mut open_ports: Vec<(u16, String)> = Vec::new();

    // Filter for open ports and extract banners
    for port in result {
        if let Some(banner) = &port.1 {
            open_ports.push((port.0, banner.trim().to_string()));
        }
    }

    println!("{}{}{}", "[".truecolor(223, 93, 108), "Open Ports".truecolor(92, 170, 180), "]".truecolor(223, 93, 108));

    if open_ports.is_empty() {
        println!("{}", "No open ports found.".red());
    } else {
        // Display each open port with its banner
        for port in open_ports {
            println!("{}: {}", port.0.to_string().green(), if port.1.is_empty() { "No banner".truecolor(92, 170, 180) } else { port.1.truecolor(92, 170, 180)});
        }
    }

}



/// Outputs scan results as formatted JSON to stdout.
///
/// # Arguments
/// * `result` - Slice of (port, banner) tuples where `Some` indicates an open port
/// * `target` - Target IP address string
/// * `ports_scanned` - Total number of ports scanned
pub fn output_json(result: &[(u16, Option<String>)], target: &str, ports_scanned: usize) {
    // filter for open ports and convert to OpenPort structs
    let open_ports: Vec<OpenPort> = result.iter().filter_map(|(port, banner)| {
        banner.as_ref().map(|b| OpenPort {
            port: *port,
            banner: b.trim().to_string(),
        })
    }).collect();

    let scan_result = ScanResult {
        target: target.to_string(),
        ports_scanned,
        open_ports,
    };

    let json = serde_json::to_string_pretty(&scan_result).unwrap();
    println!("{}", json);
}
