mod util;
use std::borrow::BorrowMut;
use std::net::*;
use std::io::prelude::*;
use std::time::Duration;
use util::multi_thread::thread_pool::multi_thread::*;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    
    for stream in listener.incoming() {
        
        let stream = stream.unwrap();
        println!("Connection established!");

        handle_connection(&stream);
        //stream.shutdown(Shutdown::Both);
    }
}
fn handle_connection(mut stream: &TcpStream) {

    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    
    let contents = String::from("RESPONSE");
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );
    
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    
}