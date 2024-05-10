use super::super::util;
use async_std::{
    io::BufReader,
    net::{TcpListener, TcpStream, ToSocketAddrs},
    prelude::*,
    task,
};
use futures::{channel::mpsc, FutureExt,SinkExt};
use std::{
    borrow::{Borrow, BorrowMut}, collections::hash_map::{Entry, HashMap}, future::Future, sync::Arc
};
use util::types::queue_type::queue_type as q_type;
use super::super::util::types::queue_type::queue_type::{Receiver,Sender};
use crate::util::{multi_thread::thread_pool::multi_thread::spawn_and_log_error, socket::socket_manager::socket_manager::*};
use futures::select;
pub async fn broker_loop(events: Receiver<SocketObject>)
{
    let (disconnect_sender, mut disconnect_receiver) = mpsc::unbounded::<(String,Receiver<String>)>();
    let mut connections:HashMap<String,Sender<String>> = HashMap::new();
    let mut events = events.fuse();
    loop{
        let event = select!
        {
            event = events.next().fuse() => match event{
                Some(event) => event,
                None => break,
            },
            disconnect = disconnect_receiver.next().fuse() => {
                let (name,_pending_msg) = disconnect.unwrap();
                assert!(connections.remove(&name).is_some());
                continue;
            }
        };
        match event {
            SocketObject::Connection(connection) => {
                
                match connections.entry(connection.pid.clone()) {
                    Entry::Occupied(..) =>(),
                    Entry::Vacant(entry) => {
                        let (client_sender,mut client_receiver) = mpsc::unbounded();
                        entry.insert(client_sender);
                        let mut disconnect_sender = disconnect_sender.clone(); // 이 블럭에서 shadowing...
                        spawn_and_log_error(async move {
                            let res = connection_writer_loop(&mut client_receiver,connection.stream,connection.shutdown).await;
                            disconnect_sender.send((connection.pid,client_receiver)).await.unwrap();
                            res
                        });
                    }
                }
            },
            SocketObject::NormalMessage(msg) => {
                for addr in msg.get_destnations() {
                    if let Some(mut sender) = connections.get(&addr) {
                        let stl = match msg.get_socket_object() {
                                SocketObject::NormalMessage(msg) => msg.get_msg(),
                                _ => unreachable!(),
                            };   
                        sender.send(stl).await.unwrap();
                    }
                }
            }
        }
    }
}


/// 연결된 클라이언트에게 메시지를 보내는 루프
/// 
/// 여기서 tcp io 작업을 수행한다.
pub async fn connection_writer_loop(
    messages: &mut Receiver<String>,
    stream: Arc<TcpStream>,
    shutdown: Receiver<Void>,
) -> q_type::Result<()> {
    let mut stream = &*stream;
    let mut messages = messages.fuse();
    let mut shutdown = shutdown.fuse();
    loop {
        select! {
            msg = messages.next().fuse() => match msg {
                Some(msg) => stream.write_all(msg.as_bytes()).await?,
                None => break,
            },
            void = shutdown.next().fuse() => match void {
                Some(void) => match void {},
                None => break,
            }
        }
    }
    Ok(())
}
