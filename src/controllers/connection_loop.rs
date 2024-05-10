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
use util::socket::socket_manager::socket_manager::{Connection,SocketObject,BrokerMessage,Void,generate_broker_message,generate_connect_broker_message};
use util::types::queue_type::queue_type as q_type;
pub async fn connection_loop(
    mut broker: Sender<BrokerMessage>,
    stream: TcpStream) -> q_type::Result<()> 
{
    let stream = Arc::new(stream);
    let reader = BufReader::new(&*stream);
    let mut lines = reader.lines();

    let name = match lines.next().await {
        Some(Ok(name)) => name,
        _ => return Err("Failed to read from socket".into()),
    };
    let (shutdown_sender, shutdown_receiver) = mpsc::unbounded::<Void>();
    let connection = Connection {
        pid: name.clone(), // 걍 소유권 복붙.
        stream: stream.clone(),
        shutdown: shutdown_receiver,
    };
    let sent_event = broker.send( 
        generate_connect_broker_message(
        connection
    ));

    Ok(())
}