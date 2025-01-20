use clap::Parser;
use rmap::{Args, Task};
use std::net::SocketAddrV4;
use std::sync::mpsc::{self, sync_channel};
use std::thread;

fn main() {
    let args = Args::parse();

    println!("rmap - a rust TCP portscanner");

    let (results_tx, results_rx) = sync_channel(20);

    let (worker, wtx, _wrx) = rmap::Worker::new(results_tx.clone());

    thread::spawn(move || { worker.main(); });

    for port in args.ports {
        let _ = wtx.send(rmap::ManagerMessage::Task(Task{
            addr: SocketAddrV4::new(args.ip, port),
            uuid: uuid::Uuid::new_v4()
        }));
    }

    'main: loop {
        match results_rx.try_recv() {
            Ok(a) => {
                dbg!(a);
                //break 'main;
            },
            Err(e) => {
                match e {
                    mpsc::TryRecvError::Empty => {},
                    mpsc::TryRecvError::Disconnected => {
                        panic!("Sender died!");
                    }
                }
            }
        };
    }
}
