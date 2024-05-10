use crate::util::socket::socket_manager::socket_manager::NormalMessage;

use super::super::util;
use futures::SinkExt;
use async_std::{
    io::BufReader,
    net::{TcpListener, TcpStream, ToSocketAddrs},
    prelude::*,
    task,
};
use futures::channel::mpsc;

use std::{
    collections::hash_map::{Entry, HashMap},
    future::Future,
    sync::Arc,
};

use util::types::queue_type::queue_type::{Sender, Receiver};
use util::socket::socket_manager::socket_manager::{Connection,SocketObject,Void};
use util::types::queue_type::queue_type as q_type;
pub async fn connection_loop(
    mut broker: Sender<SocketObject>,
    stream: TcpStream) -> q_type::Result<()> 
{
    let stream: Arc<TcpStream> = Arc::new(stream);
    let connector_address =  match stream.clone().peer_addr() {
        Ok(addr) => addr,
        Err(_) => return Err("Failed to get peer address".into()),   
    };
    let reader = BufReader::new(&*stream);
    let mut lines = reader.lines();

    let mut name = match lines.next().await {
        Some(Ok(name)) => name,
        _ => return Err("Failed to read from socket".into()),
    };
    name.push_str(connector_address.to_string().as_str());
    let (shutdown_sender, shutdown_receiver) = mpsc::unbounded::<Void>();
    let connection = Connection {
        pid: name.clone(), // 걍 소유권 복붙.
        stream: stream.clone(),
        shutdown: shutdown_receiver,
    };
    broker.send( 
        SocketObject::Connection(connection)
    ).await.unwrap();
    print!("{} conn... \n", name);
    while let Some(line) = lines.next().await {
        let line = line?;
        print!("{}: {}\n", name, line);
        let msg = NormalMessage::new(name.clone(), vec![], 0, line);
        broker.send(SocketObject::NormalMessage(msg)).await.unwrap();
    }
    Ok(())
}