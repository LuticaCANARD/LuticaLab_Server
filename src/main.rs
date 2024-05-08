mod util;
use std::borrow::BorrowMut;
use std::fmt::Error;
use std::net::*;
use std::io::prelude::*;
use std::time::Duration;
use util::multi_thread::thread_pool::multi_thread::*;
use async_std::{
    io::{BufReader, BufWriter},
    net::{TcpListener, TcpStream, ToSocketAddrs},
    prelude::*,
    task,
};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
fn main() {

}
fn spawn_and_log_error<F>(fut: F) -> task::JoinHandle<()>
where
    F: Future<Output = Result<()>> + Send + 'static,
{
    task::spawn(async move {
        if let Err(e) = fut.await {
            eprintln!("{}", e)
        }
    })
}

async fn connection_loop(mut stream: TcpStream) -> Result<()> {
    let reader: BufReader<&TcpStream> = BufReader::new(&stream);
    let mut writer: BufWriter<&TcpStream> = BufWriter::new(&stream);

    let name: String = match reader.lines().next().await {
        None => Err("peer disconnected immediately")?,
        Some(line) => line?,
    };
    println!("name = {}", name);

    // write response to client
    let response: &str = "HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nHello, world!";
    writer.write_all(response.as_bytes()).await?;
    writer.flush().await?;

    Ok(())
}
async fn accept_loop(addr:impl ToSocketAddrs) -> Result<()> {
    let listener = TcpListener::bind(addr).await?;
    let mut incoming:async_std::net::Incoming = listener.incoming();
    while let Some(stream) = incoming.next().await {
        let stream = stream?;

        let handler :task :: JoinHandle<()> = spawn_and_log_error(connection_loop(stream));
    }
    Ok(())
}