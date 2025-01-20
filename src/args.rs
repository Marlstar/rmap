use clap::Parser;

#[derive(Debug, Parser)]
#[command(
    name = "rmap",
    author = "Marley Reeves",
    version,
    about = "rmap: A simple TCP portscanner",
    long_about = None
)]
pub struct Args {
    /// Target ip
    pub ip: std::net::Ipv4Addr,

    #[arg(short, long)]
    /// Target port
    pub port: u16
}
