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
    use futures::{select, FutureExt};    
    pub struct ThreadPool {
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
    ///
    /// Worker는 추적 id와 업무인 work, 실행중인 thread를 가진다.
    struct Worker { 
        id: usize,
        thread: thread::JoinHandle< Result<(),Error>>,
        work : fn() -> Result<(),Error>,
    }
    
    impl Worker {
        ///
        /// 생성과 함께 thread 시작함.
        fn new(&self,id: usize,work:fn() -> Result<(),Error>) -> Worker { 
            let thread = thread::spawn(work);

            Worker {
                id,
                thread,
                work
            }
        }
    }
}