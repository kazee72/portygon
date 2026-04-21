use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use portygon::{cli::Cli, output, ports, scanner, target};
use std::net::IpAddr;
use std::sync::Arc;
use tokio::sync::Semaphore;

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    let ip: IpAddr = match target::resolve_target(&args.target).await {
        Ok(addr) => addr,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    let parsed_ports: Vec<u16> = ports::parse_ports(&args.ports);

    if parsed_ports.is_empty() {
        eprintln!("Error: No valid ports specified");
        std::process::exit(1);
    }

    let total_ports = parsed_ports.len();

    let progress_bar = ProgressBar::new(parsed_ports.len() as u64);
    progress_bar.set_style(
        ProgressStyle::with_template("[{spinner}] Scanning... {pos}/{len} ports {percent}%")
            .unwrap()
            .tick_chars("||//--\\\\"),
    );

    progress_bar.enable_steady_tick(std::time::Duration::from_millis(100));

    let mut results: Vec<(u16, Option<String>)> = Vec::new();

    if args.stealth {
        for port in parsed_ports {
            let scan_result = scanner::scan(ip, &args.target, port).await;
            results.push((port, scan_result));
            progress_bar.inc(1);

            let delay = rand::random_range(2..=5);
            tokio::time::sleep(std::time::Duration::from_secs(delay)).await;
        }
    } else {
        let mut tasks = Vec::with_capacity(parsed_ports.len());
        // limit concurrent connections to avoid overwhelming the target
        let semaphore = Arc::new(Semaphore::new(100));

        // spawn async tasks for each port
        for port in parsed_ports {
            let pb_clone = progress_bar.clone();
            let semaphore = semaphore.clone();
            let target = args.target.clone();

            tasks.push(tokio::spawn(async move {
                let _permit = semaphore.acquire().await.expect("semaphore closed");
                let scan_result = scanner::scan(ip, &target, port).await;
                pb_clone.inc(1);
                (port, scan_result)
            }));
        }
        // collect results from all tasks
        for task in tasks {
            match task.await {
                Ok(result) => results.push(result),
                Err(e) => eprintln!("Task failed: {}", e),
            }
        }
    }

    progress_bar.finish();

    results.sort_by_key(|(port, _)| *port);

    if args.json {
        output::output_json(&results, &args.target, total_ports);
    } else {
        output::display_results(&results);
    }
}
