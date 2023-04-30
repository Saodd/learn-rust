use std::ops::Deref;
use std::panic::catch_unwind;
use std::sync::mpsc::{channel, Receiver, SendError, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::{JoinHandle, Thread};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    sender: Sender<Job>,
    workers: Vec<Worker>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        let (sender, receiver) = channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, receiver.clone()))
        }

        return ThreadPool { sender, workers };
    }

    pub fn execute<F>(&self, f: F) -> Result<(), SendError<Job>>
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.send(Box::new(f))
    }
}

struct Worker {
    id: usize,
    thread: JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Self {
        let t = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            job()
        });
        return Self { id, thread: t };
    }
}
