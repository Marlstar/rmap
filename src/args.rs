use clap::Parser;
use std::{ops::Deref, time::Duration};

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

    #[arg(short = 't', long, default_value_t = DurationWrap(Duration::from_millis(200)), value_parser = parse_duration)]
    /// Timeout on TCP read operations for successful connections
    pub read_timeout: DurationWrap,
}
impl Args { pub fn get() -> &'static Self {
        use std::sync::OnceLock;
        static ARGS: OnceLock<Args> = OnceLock::new();

        let _ = ARGS.get_or_init(Args::parse);

        return ARGS.get().unwrap();
    }
}

#[derive(Debug, Clone)]
pub struct DurationWrap(Duration);
impl std::fmt::Display for DurationWrap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}",
            self.0
        )
    }
}
impl Deref for DurationWrap {
    type Target = Duration;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
fn parse_duration(input: &str) -> Result<DurationWrap, parse_duration::parse::Error> {
    return Ok(DurationWrap(parse_duration::parse(input)?));
}
