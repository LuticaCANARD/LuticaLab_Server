pub mod socket_manager 
{
    use std::{
        collections::hash_map::{Entry, HashMap},
        future::Future,
        sync::Arc,
    };
    use async_std::{
        io::BufReader,
        net::{TcpListener, TcpStream, ToSocketAddrs},
        prelude::*,
        task,
    };
    use super::super::types::queue_type::{Sender, Receiver};
    #[derive(Debug)]
    enum Void {}

    pub enum SocketObject // 소켓 접선에 따라 생성되는 오브젝트들
    {
        Connection{ // 연결
            pid : String, // 보낸 사람.
            stream : Arc<TcpStream>, // 소켓 스트림
            shutdown : Receiver<Void>,
        },
        NormalMessage // 일반 메시지
        {
            from : String, // 보낸 사람
            to : Vec<String>, // 대상
            order : i64, // 명령어
            message : String, // 메시지
        }
    }
    
}

