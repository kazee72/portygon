use clap::Parser;
use portygon::{cli::Cli, ports, scanner, output};
use indicatif::{self, ProgressBar, ProgressStyle};
use tokio::sync::Semaphore;
use std::sync::Arc;



#[tokio::main]
async fn main() {
    let args = Cli::parse();
    
    let parsed_ports: Vec<u16> = ports::parse_ports(&args.ports);

    let mut tasks = Vec::with_capacity(parsed_ports.len());

    let progress_bar = ProgressBar::new(parsed_ports.len() as u64);
    progress_bar.set_style(ProgressStyle::with_template("[{spinner}] Scanning... {percent}%")
        .unwrap()
        .tick_chars("||//--\\\\")
    );

    progress_bar.enable_steady_tick(std::time::Duration::from_millis(100));

    let semaphore = Arc::new(Semaphore::new(200));

    for port in parsed_ports {

        let static_target_str = args.target.clone();
        let pb_clone = progress_bar.clone();
        let semaphore = semaphore.clone();

        tasks.push(tokio::spawn(async move {
            let _permit = semaphore.acquire().await.unwrap();
            let scan_result = scanner::scan(&static_target_str, port).await;
            pb_clone.inc(1);
            (port, scan_result)
        }));
    }

    let mut results: Vec<(u16, bool)> = Vec::new();

    for task in tasks {
        let output = task.await;
        results.push(output.unwrap());
    }

    progress_bar.finish();

    output::display_results(&results);
}
