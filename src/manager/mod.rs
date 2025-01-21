mod worker_info;
use worker_info::WorkerInfo;

use std::{net::{Ipv4Addr, SocketAddrV4}, sync::mpsc::sync_channel};

use hashbrown::{HashMap, HashSet};
use uuid::Uuid;
use crate::{worker::TcpResult, ManagerMessage, Task, TaskResult, Worker, WorkerMessage};

use colored::Colorize;

#[derive(Debug)]
pub struct Manager {
    tasks: HashMap<Uuid, Task>,
    todo: Vec<Uuid>,
    assigned: HashSet<Uuid>,
    done: HashSet<Uuid>,
    workers: HashMap<Uuid, WorkerInfo>,
    free: Vec<Uuid>,
    busy: HashSet<Uuid>,
    results_tx: crate::ResultsTX,
    results_rx: crate::ResultsRX,
    results: HashMap<Uuid, TcpResult>
}
impl Manager { // Constructor and its functions
    pub fn new(ips: Vec<Ipv4Addr>, ports: Vec<u16>, pool_size: usize) -> Self {
        // Generate the tasks
        let tasks = Self::create_tasks(ips, ports);
        let num_tasks = tasks.len();
        // Auto-shrink the pool size if there are more workers than tasks
        let pool_size = if pool_size > tasks.len() { tasks.len() } else { pool_size };

        let (results_tx, results_rx) = sync_channel(pool_size);

        let mut manager = Self {
            tasks: HashMap::new(),
            todo: Vec::with_capacity(num_tasks),
            assigned: HashSet::with_capacity(num_tasks),
            done: HashSet::with_capacity(num_tasks),
            workers: HashMap::with_capacity(pool_size),
            free: Vec::with_capacity(pool_size),
            busy: HashSet::with_capacity(pool_size),
            results_tx,
            results_rx,
            results: HashMap::with_capacity(tasks.len())
        };

        for task in tasks {
            manager.todo.push(task.uuid);
            manager.tasks.insert(task.uuid, task);
        }

        // Populate the worker pool
        for _ in 0..pool_size {
            let (worker, tx, rx) = Worker::new(manager.results_tx.clone());
            let handle = std::thread::spawn(move || { worker.run() });
            let worker_info = WorkerInfo {
                handle,
                id: Uuid::new_v4(),
                tx,
                rx
            };
            manager.free.push(worker_info.id);
            manager.workers.insert(worker_info.id, worker_info);
        }

        return manager;
    }

    fn create_tasks(ips: Vec<Ipv4Addr>, ports: Vec<u16>) -> Vec<Task> {
        let mut tasks: Vec<Task> = Vec::with_capacity(ips.len() * ports.len());

        for ip in ips {
            for port in &ports {
                tasks.push(Self::create_task(ip, *port));
            }
        }

        return tasks;
    }

    fn create_task(ip: Ipv4Addr, port: u16) -> Task {
        Task {
            addr: SocketAddrV4::new(ip, port),
            uuid: uuid::Uuid::new_v4()
        }
    }
}
impl Manager { // Main running logic
    pub fn run(&mut self) {
        println!("Running {} tasks.\n", self.tasks.len());
        'main: loop {
            if self.all_tasks_complete() { break 'main }
            self.receive_and_handle_messages();
            self.receive_results();
            self.assign_tasks();
        }
        println!("DONE!");
        self.exit();
    }

    pub fn all_tasks_complete(&self) -> bool {
        return self.done.len() == self.tasks.len();
    }

    pub fn exit(&mut self) {
        for (_, worker) in self.workers.iter() {
            worker.send_message(ManagerMessage::Exit);
        }
    }

    fn receive_and_handle_messages(&mut self) {
        let mut messages: Vec<(Uuid, WorkerMessage)> = vec![];
        for (id, worker) in self.workers.iter() {
            let msgs = worker.receive_messages();
            for message in msgs {
                messages.push((*id, message));
            }
        }

        for msg in messages { self.handle_message(msg.0, msg.1); }
    }


    fn handle_message(&mut self, worker_id: Uuid, message: WorkerMessage) {
        match message {
            WorkerMessage::Done => {
                self.busy.remove(&worker_id);
                self.free.push(worker_id);
            }
        }
    }

    fn receive_results(&mut self) {
        while let Ok(result) = self.results_rx.try_recv() {
            self.output_result(&result);
            self.assigned.remove(&result.uuid);
            self.done.insert(result.uuid);
            self.results.insert(result.uuid, result.result);
        }
    }

    fn output_result(&self, result: &TaskResult) {
        let addr = self.tasks.get(&result.uuid).unwrap().addr;
        let ip = addr.ip().to_string().white();
        let port = addr.port().to_string().bright_magenta();
        let spacing_len = 23 - addr.to_string().chars().count();
        let spacing: String = (0..=spacing_len).map(|_| ' ').collect();
        println!(
            "{ip}:{port}{spacing}{}",
            match result.result {
                TcpResult::Open => "OPEN".bright_green(),
                TcpResult::Closed => "CLOSED".bright_red(),
            }
        );
    }

    fn assign_tasks(&mut self) {
        let num_free = self.free.len();
        let num_tasks = self.todo.len();
        let num_assignments = num_free.min(num_tasks);

        for _ in 0..num_assignments {
            let worker = self.free.pop().unwrap();
            let task = self.todo.pop().unwrap();
            self.assign_worker(worker, task);
        }
    }

    fn assign_worker(&mut self, worker_id: Uuid, task_id: Uuid) {
        let worker = self.workers.get(&worker_id).unwrap();
        let task = self.tasks.get(&task_id).unwrap();
        worker.send_task(task.clone());
        self.assigned.insert(task_id);
        self.busy.insert(worker_id);
    }
}
