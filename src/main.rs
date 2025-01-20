use clap::Parser;
use rmap::Args;

fn main() {
    let args = Args::parse();
    let mut manager = rmap::Manager::new(vec![args.ip], args.ports, args.pool_size);
    manager.run();
}
