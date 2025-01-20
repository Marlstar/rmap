use rmap::worker::TcpChecker;

fn main() {
    println!("rmap - a rust TCP portscanner");

    let checker = TcpChecker::new(std::net::Ipv4Addr::new(192,168,50,1), 443);
    dbg!(checker.check());
}
