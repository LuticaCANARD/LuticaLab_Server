
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

    pub struct Connection{ // 연결
        pid : String, // 보낸 사람.
        stream : Arc<TcpStream>, // 소켓 스트림
        shutdown : Receiver<Void>,
    }
    pub enum SocketObject // 소켓 접선에 따라 생성되는 오브젝트들
    {
        NormalMessage(NormalMessage), // 일반 메시지
        BrokerMessage(BrokerMessage), // 브로커에 전달되는 메시지
    }
    pub struct NormalMessage {
        from : String, // 보낸 사람
        to : Vec<String>, // 대상
        order : i64, // 명령어
        message : String, // 메시지
    }
    pub struct BrokerMessage
    {
        from : String,
        to:Vec<String>,
        order:i64,
        reference : Arc<SocketObject> // 다른 메시지를 가르키는 스마트포인터!
    }

    pub trait MsgTrait {
        fn get_from(&self)->String;
        fn get_order(&self)->i64;
    }
    impl MsgTrait for NormalMessage {
        fn get_from(&self)->String {
            return self.from
        }
        fn get_order(&self)->i64 {
            return self.order
        }
    }
    
}

