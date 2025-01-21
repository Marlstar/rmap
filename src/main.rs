use clap::Parser;
use rmap::Args;

fn main() {
    let args = Args::parse();
    let mut manager = rmap::Manager::new(args.ips, args.ports, args.pool_size);
    manager.run();
}
