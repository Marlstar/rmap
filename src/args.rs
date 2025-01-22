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
    #[arg(value_delimiter=',')]
    pub ips: Vec<std::net::Ipv4Addr>,

    #[arg(short, long, value_delimiter=',')]
    /// Target port
    pub ports: Vec<u16>,

    #[arg(short = 'w', long, default_value_t = 5)]
    /// Worker pool size
    pub pool_size: usize,
}
impl Args {
    pub fn get() -> &'static Self {
        use std::sync::OnceLock;
        static ARGS: OnceLock<Args> = OnceLock::new();

        let _ = ARGS.get_or_init(Args::parse);

        return ARGS.get().unwrap();
    }
}
