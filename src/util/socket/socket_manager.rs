
pub mod socket_manager 
{
    use std::{
        collections::hash_map::{Entry, HashMap}, future::Future, io::Read, sync::Arc
    };
    use async_std::{
        io::BufReader,
        net::{TcpListener, TcpStream, ToSocketAddrs},
        prelude::*,
        task,
    };
    use super::super::types::queue_type::{Sender, Receiver};
    #[derive(Debug)]
    pub enum Void {}
    pub struct Connection{ // 연결
        pub pid : String, // 보낸 사람.
        pub stream : Arc<TcpStream>, // 소켓 스트림
        pub shutdown : Receiver<Void>,
    }
    pub enum SocketObject // 소켓 접선에 따라 생성되는 오브젝트들
    {
        Connection(Connection), // 연결
        NormalMessage(NormalMessage), // 일반 메시지
    }
    #[derive(Clone)]
    pub struct NormalMessage {
        from : String, // 보낸 사람
        to : Vec<String>, // 대상
        order : i64, // 명령어
        pub message : String, // 메시지
    }
    impl NormalMessage {
        pub fn get_msg(&self)->String {
            return self.message.clone()
        }
        pub fn new(from:String,to:Vec<String>,order:i64,message:String)->Self {
            return NormalMessage {
                from,
                to,
                order,
                message
            }
        }
    }

    pub trait MsgTrait {
        fn get_from(&self)->String;
        fn get_order(&self)->i64;
        fn get_socket_object(&self)->SocketObject;
        fn get_destnations(&self)->Vec<String>;
    }
    impl MsgTrait for NormalMessage {
        fn get_from(&self)->String {
            return self.from.clone()
        }
        fn get_order(&self)->i64 {
            return self.order
        }
        fn get_socket_object(&self)->SocketObject {
            return SocketObject::NormalMessage(self.clone())
        }
        fn get_destnations(&self)->Vec<String> {
            return self.to.clone()
        }
    }
    
}

