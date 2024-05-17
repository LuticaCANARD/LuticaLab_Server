use controllers::accept_controller::accept_loop;
mod util;
use std;

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
mod controllers;


// https://book.async.rs/tutorial/handling_disconnection
fn main() {
    env!("RUST_BACKTRACE","1");
    let _res = task::block_on(accept_loop("127.0.0.1:8080"));
}

