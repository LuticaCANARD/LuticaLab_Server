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
pub async fn connection_loop(
    mut broker: Sender<Event>,
    stream: TcpStream) -> q_type::Result<()> 
{

}