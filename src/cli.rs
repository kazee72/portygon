use clap::Parser;

#[derive(Parser)]
#[command(name = "portygon", about = "Port scanner", version)]
pub struct Cli {
    #[arg(help = "Target ip")]
    pub target: String,

    #[arg(short, long, default_value = "1-1024", help = "Ports to scan")]
    pub ports: String,

    #[arg(
        short,
        long,
        help = "Stealth mode: sequential scan with random delays (2-5s)"
    )]
    pub stealth: bool,

    #[arg(short, long, help = "Output results as JSON")]
    pub json: bool,
}
