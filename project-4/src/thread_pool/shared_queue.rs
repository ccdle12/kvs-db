use super::ThreadPool;
use crate::Result;
use crossbeam::channel::{self, Receiver, Sender};
use std::thread;

pub struct SharedQueueThreadPool {
    tx: Sender<Box<dyn FnOnce() + Send + 'static>>,
}

impl ThreadPool for SharedQueueThreadPool {
    fn new(threads: u32) -> Result<Self> {
        // 1. Create a Channel tx, rx that refs a Box<dyn FnOnce() + Send + 'static>.
        // 2. For loop from 0 to threads
        // 3. Clone rx into TaskReceiver.
        let (tx, rx) = channel::unbounded::<Box<dyn FnOnce() + Send + 'static>>();

        for i in 0..threads {
            println!("DEBUG: Creating thread: {}", i);
            let task_rx = TaskReceiver(rx.clone());
            thread::Builder::new()
                .spawn(move || run_tasks(task_rx))
                .unwrap();
        }

        Ok(SharedQueueThreadPool { tx })
    }

    fn spawn<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.tx
            .send(Box::new(job))
            .expect("The thread pool has no thread.");
    }
}

// Wraps crossbeam Receiver.
#[derive(Clone)]
struct TaskReceiver(Receiver<Box<dyn FnOnce() + Send + 'static>>);

// Trait that runs when the TaskReceiver is dropped in memory.
impl Drop for TaskReceiver {
    fn drop(&mut self) {
        // When being dropped from memory, if the thread panics, spawn a new
        // thread to replace it.
        println!("DEBUG: DROP is called on a thread");
        if thread::panicking() {
            println!("DEBUG: DROP thread has panicked creating new thread");
            let rx = self.clone();
            if let Err(e) = thread::Builder::new().spawn(move || run_tasks(rx)) {
                panic!("Failed to spawn thread!: {}", e);
            }
        }
    }
}

fn run_tasks(rx: TaskReceiver) {
    loop {
        match rx.0.recv() {
            Ok(task) => {
                println!("DEBUG: Running task");
                task();
            }
            Err(_) => println!("Thread exits because the thread pool is destroyed."),
        }
    }
}
