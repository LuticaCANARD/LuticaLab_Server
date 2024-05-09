
mod util;
use std;

use async_std::{
    io::BufReader,
    net::{TcpListener, TcpStream, ToSocketAddrs},
    prelude::*,
    task,
};

use std::{
    collections::hash_map::{Entry, HashMap},
    future::Future,
    sync::Arc,
};


// https://book.async.rs/tutorial/handling_disconnection
fn main() {
    task::block_on(accept_loop("127.0.0.1:8080")).unwrap();
}

fn accept_loop(addr:impl ToSocketAddrs) -> Future<util::types::queue_type::queue_type::Result<()>>
{

}