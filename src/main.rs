use clap::Parser;
use rmap::worker::TcpChecker;
use rmap::Args;

fn main() {
    let args = Args::parse();

    println!("rmap - a rust TCP portscanner");

    let checker = TcpChecker::new(args.ip, args.port);
    dbg!(checker.check());
}
