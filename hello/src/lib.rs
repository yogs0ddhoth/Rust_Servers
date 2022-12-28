use std::{
    sync::{
        mpsc::{
            self, 
            Receiver, 
            Sender
        },
        Arc, 
        Mutex,
    },
    thread,
};

type Job = Box<dyn FnOnce() + Send + 'static>; // type alias to send closures to threads

/// # ThreadPool
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<Sender<Job>>,
}
impl ThreadPool {
    /// # Constructor Method
    ///
    /// create a thread pool with the specified number of threads in it
    ///
    /// # Panics
    /// size must be larger than 0
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel(); // create a channel
        let receiver = Arc::new(Mutex::new(receiver)); // wrap the receiver in Arc<Mutex<T>> to allow multiple threads to safely use the receiver

        /* create and store as many workers as specified in params, each with a receiver */
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender: Some(sender) }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender
            .as_ref()
            .unwrap()
            .send(Box::new(f)) // wrap the method f in a Box (aliased as a Job) and send it down the channel
            .unwrap();
    }
}
impl Drop for ThreadPool {
    /* Cleanup */
    fn drop(&mut self) {
        drop(self.sender.take()); // drop the sender

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            
            // destructure and join the threads
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        Worker {
            id,
            thread: Some(
                // NOTE in a production build, thread::Builder, which returns a Result would be preferable
                thread::spawn(move || loop {
                    let message = receiver.lock().unwrap().recv(); // attempt to wait for a message from the sender, temporary values, like the lock, are dropped when the let statement ends
                    match message {
                        Ok(job) => {
                            println!("Worker {id} got a job; executing.");
    
                            job();
                        }
                        Err(_) => {
                            println!("Worker {id} disconnected; shutting down.");
                            break;
                        }
                    }
                }) 
            )
        }
    }
}
