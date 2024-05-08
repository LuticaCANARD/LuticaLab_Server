pub mod multi_thread
{
    use std::{fmt::Error, thread};
    use async_std::{
        io::BufReader,
        net::{TcpListener, TcpStream, ToSocketAddrs},
        prelude::*,
        task,
    };
    use futures::channel::mpsc;
    use futures::sink::SinkExt;
    use futures::{select, FutureExt};    pub struct ThreadPool {
        workers: Vec<Worker>,
    }
    
    impl ThreadPool {
        // --생략--
        pub fn new(size: usize) -> ThreadPool {
            assert!(size > 0);
            
            let mut workers = Vec::with_capacity(size);
            
            for id in 0..size {
                //workers.push(Worker::new(id));
            }
            
            ThreadPool {
                workers
            }
        }
    }
    
    struct Worker {
        id: usize,
        thread: thread::JoinHandle< Result<(),Error>>,
        work : fn() -> Result<(),Error>,
    }
    
    impl Worker {
        fn new(&self,id: usize) -> Worker { 
            let thread = thread::spawn(self.work);

            Worker {
                id,
                thread,
                work : || -> Result<(),Error> {
                    Ok(())
                }
            }
        }
    }
}