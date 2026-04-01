use clap::Parser;
use portygon::{cli::Cli, ports, scanner, output};
use indicatif::{self, ProgressBar, ProgressStyle};
use tokio::sync::Semaphore;
use std::net::IpAddr;
use std::sync::Arc;
use std::collections::HashSet;



#[tokio::main]
async fn main() {
    let args = Cli::parse();

    let ip: IpAddr = match args.target.parse() {
        Ok(addr) => addr,
        Err(_) => {
            eprintln!("Error: '{}' is not a valid IP address", args.target);
            std::process::exit(1);
        }
        
    };

    // ports that require an HTTP request for banner grabbing
    let http_ports: HashSet<u16> = HashSet::from([80, 443, 8080, 8443, 8000, 8888, 3000, 3001, 5000, 5173, 4200, 8081, 9090, 9443]);
    
    let parsed_ports: Vec<u16> = ports::parse_ports(&args.ports);

    let mut tasks = Vec::with_capacity(parsed_ports.len());

    let progress_bar = ProgressBar::new(parsed_ports.len() as u64);
    progress_bar.set_style(ProgressStyle::with_template("[{spinner}] Scanning... {pos}/{len} ports {percent}%")
        .unwrap()
        .tick_chars("||//--\\\\")
    );

    progress_bar.enable_steady_tick(std::time::Duration::from_millis(100));

    // limit concurrent connections to avoid overwhelming the target
    let semaphore = Arc::new(Semaphore::new(100));
    let http_ports_arc = Arc::new(http_ports);

    // spawn async tasks for each port
    for port in parsed_ports {

        let pb_clone = progress_bar.clone();
        let semaphore = semaphore.clone();
        let http_ports = http_ports_arc.clone();

        tasks.push(tokio::spawn(async move {
            let _permit = semaphore.acquire().await.unwrap();
            let scan_result = scanner::scan(ip, port, &http_ports).await;
            pb_clone.inc(1);
            (port, scan_result)
        }));
    }

    let mut results: Vec<(u16, Option<String>)> = Vec::new();

    // collect results from all tasks
    for task in tasks {
        let output = task.await;
        results.push(output.unwrap());
    }

    progress_bar.finish();

    output::display_results(&results);
}
