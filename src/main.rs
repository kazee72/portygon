use clap::Parser;
use portygon::{cli::Cli, ports, scanner};



#[tokio::main]
async fn main() {
    let args = Cli::parse();
    
    let parsed_ports: Vec<u16> = ports::parse_ports(&args.ports);

    let mut tasks = Vec::with_capacity(parsed_ports.len());

    for port in parsed_ports {
        let static_target_str = args.target.clone();
        tasks.push(tokio::spawn(async move {
            let scan_result = scanner::scan(&static_target_str, port).await;
            (port, scan_result)
        }));
    }

    let mut results: Vec<(u16, bool)> = Vec::new();

    for task in tasks {
        let output = task.await;
        results.push(output.unwrap());
    }
}