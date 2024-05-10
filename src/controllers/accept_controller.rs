
use super::{super::util, connection_loop};
use async_std::{
    io::BufReader,
    net::{TcpListener, TcpStream, ToSocketAddrs},
    prelude::*,
    task,
};
use super::broker_loop::broker_loop;
use futures::channel::mpsc::{self, UnboundedSender};

use std::{
    collections::hash_map::{Entry, HashMap},
    future::Future,
    sync::Arc,
};
use util::multi_thread::thread_pool::multi_thread::spawn_and_log_error;
use connection_loop::connection_loop;
pub async fn accept_loop(addr:impl ToSocketAddrs) ->  util::types::queue_type::queue_type::Result<()>
{
    let listener = TcpListener::bind(addr).await?;
    let (q_broker_sender,q_broker_receiver) = mpsc::unbounded();
    let broker_handle = task::spawn(
        broker_loop(q_broker_receiver)
    );
    let mut incomming = listener.incoming();
    while let Some(stream) = incomming.next().await{
        spawn_and_log_error(
            connection_loop(
                q_broker_sender.clone(),
                stream?
            )
        );
    }

    drop(q_broker_sender);
    broker_handle.await;
    Ok(())

}
