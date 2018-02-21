use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

pub mod worker;

pub struct ThreadPool {
    workers: Vec<worker::Worker>,
    sender: mpsc::Sender<worker::Message>,
}

pub struct PoolCreationError {
    pub error: String,
}

impl ThreadPool {
    pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size < 1 {
            let error = String::from(format!("{} is not a valid amount of threads!", size));
            return Err(PoolCreationError{error});
        }

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(worker::Worker::new(id, Arc::clone(&receiver)))
        }

        Ok(ThreadPool {
            workers,
            sender,
        })
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);

        self.sender.send(worker::Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &mut self.workers {
            self.sender.send(worker::Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
