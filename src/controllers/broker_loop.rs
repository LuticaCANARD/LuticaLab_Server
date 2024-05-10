use super::super::util;
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
use util::types::queue_type::queue_type as q_type;
use super::super::util::types::queue_type::queue_type::{Receiver,Sender};
use crate::util::socket::socket_manager::socket_manager::*;
pub async fn broker_loop(events: Receiver<BrokerMessage>)
{
    let (disconnect_sender, disconnect_receiver) = mpsc::unbounded::<(String,Sender<String>)>();
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
                cons
            }
        };
        }
    }
}