pub mod multi_thread
{
    use std::thread;
    pub struct ThreadPool {
        workers: Vec<Worker>,
    }
    
    impl ThreadPool {
        // --생략--
        pub fn new(size: usize) -> ThreadPool {
            assert!(size > 0);
            
            let mut workers = Vec::with_capacity(size);
            
            for id in 0..size {
                workers.push(Worker::new(id));
            }
            
            ThreadPool {
                workers
            }
        }
    }
    
    struct Worker {
        id: usize,
        thread: thread::JoinHandle<()>,
    }
    
    impl Worker {
        fn new(id: usize) -> Worker {
            let thread = thread::spawn(|| {});
            
            Worker {
                id,
                thread,
            }
        }
    }
}