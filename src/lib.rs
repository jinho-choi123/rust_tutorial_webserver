use std::{
    sync::{mpsc, Arc, Mutex},
    thread};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let mut workers: Vec<Worker> = Vec::with_capacity(size);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool{
            workers,
            sender: Some(sender)
        }
    }

    pub fn execute<F>(&self, f: F)
    where
    F: FnOnce() + Send + 'static {
        let job: Job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");
                    job();
                },
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                },
            }

        });
        Worker {
            id,
           thread: Some(thread),
        }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in &mut self.workers {

            // if thread is not None, then join
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }

        }
    }
}