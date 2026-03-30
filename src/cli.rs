use clap::Parser;

#[derive(Parser)]
#[command(name= "portygon", about = "Port scanner")]
pub struct Cli {
    #[arg(help = "Target ip")]
    pub target: String,

    #[arg(short, long, default_value = "1-1024", help = "Ports to scan")]
    pub ports: String,
}