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
pub async fn accept_loop(addr:impl ToSocketAddrs) ->  util::types::queue_type::queue_type::Result<()>
{
    let listener = TcpListener::bind(addr).await?;
    let (q_broker_sender,q_broker_receiver) = mpsc::unbounded();
    let broker_handle = task::spawn(
        super::broker_loop::broker_loop(q_broker_receiver)
    );
    let mut incomming = listener.incoming();
    while let Some(stream) = incomming.next().await{
        
        
    }

    drop(q_broker_sender);
    broker_handle.await;
    Ok(())

}
