mod util;
use std::borrow::BorrowMut;
use std::net::*;
use std::io::prelude::*;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {

        let stream = stream.unwrap();
        println!("Connection established!");
        handle_connection(stream);
    }
}
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let contents = String::from("RESPONSE");
    let req = String::from_utf8_lossy(&buffer);
    println!("{:?}",req);

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}