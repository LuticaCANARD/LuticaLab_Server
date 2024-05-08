# Rust 서버 기반코드 분석
> ORIGIN : https://book.async.rs/tutorial/handling_disconnection
## 제반

### 이벤트 객체

```rust
#[derive(Debug)]
enum Event { // 이벤트를 지정함.
    NewPeer { // TCP 객체
        name: String,
        stream: Arc<TcpStream>, // 스마트 포인터.
        shutdown: Receiver<Void>,
    },
    Message { // 메세지를 지정함
        from: String,
        to: Vec<String>, // 누구에게?
        msg: String,
    },
}

```

### 흐름 평면

```rust
use std;

use async_std::{
    io::BufReader,
    net::{TcpListener, TcpStream, ToSocketAddrs},
    prelude::*,
    task,
};
use futures::channel::mpsc;
use futures::sink::SinkExt;
use futures::{select, FutureExt};
use std::{
    collections::hash_map::{Entry, HashMap},
    future::Future,
    sync::Arc,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
type Sender<T> = mpsc::UnboundedSender<T>; // mpsc에서 쓰레드간 데이터 "전송자"
type Receiver<T> = mpsc::UnboundedReceiver<T>; // ' "수신자"

#[derive(Debug)]
enum Void {}

// https://book.async.rs/tutorial/handling_disconnection
fn main() {
    task::block_on( // 그런데 그것을 Block하여 전달함.
        accept_loop("127.0.0.1:8080") // 주어진 리소스 주소로 연결을 받음.
        ).unwrap(); 
}

async fn accept_loop(addr: impl ToSocketAddrs) -> Result<()> {
    // 연결을 수립받는 loop
    let listener = TcpListener::bind(addr).await?;
    let (broker_sender, broker_receiver) = mpsc::unbounded(); // 
    let broker_handle = task::spawn(broker_loop(broker_receiver));
    let mut incoming = listener.incoming();
    while let Some(stream) = incoming.next().await { // 뭔가 stream이 온다면!
        let stream = stream?; // 그 객체를 받는다.~> 에러시 전파
        println!("Accepting from: {}", stream.peer_addr()?); 
        spawn_and_log_error(connection_loop(
            broker_sender.clone(), // 참조 메모리를 복제해야함. 
            stream)); // 
    }
    drop(broker_sender); // 메모리 정리한다.
    broker_handle.await; // 브로커(메세지 전달자)도 대기한다.
    Ok(())
}

async fn connection_loop(
    mut broker: Sender<Event>,
    stream: TcpStream) 
    -> Result<()> {
    let stream = Arc::new(stream); // 고유포인터를 만든다.
    let reader = BufReader::new(&*stream); // 스트림의 참조로 버퍼 읽음
    let mut lines = reader.lines();

    let name = match lines.next().await {
        None => Err("peer disconnected immediately")?, // 에러나면 전파.
        Some(line) => line?,
    };
    let (_shutdown_sender, shutdown_receiver) = mpsc::unbounded::<Void>(); // 닫아주는 메세지 전달자 
    broker.send(Event::NewPeer { // 이벤트를 전달해준다.
        name: name.clone(),
        stream: Arc::clone(&stream),// 스마트포인터를 전송한여 스트림이 끊기지 아니하게 함.
        shutdown: shutdown_receiver, // 셧다운 나게 하는 전달자 저장.
    }).await.unwrap();

    while let Some(line) = lines.next().await { // 읽을때까지 존버
        let line = line?; // 
        let (dest, msg) = match line.find(':') { // 쪼갬...
            None => continue,
            Some(idx) => (&line[..idx], line[idx + 1 ..].trim()), 
        }; 
        let dest: Vec<String> = dest.split(',').map(|name| name.trim().to_string()).collect(); // 분리해서 확인한다.
        let msg: String = msg.trim().to_string(); 

        broker.send(Event::Message {
            from: name.clone(),
            to: dest,
            msg,
        }).await.unwrap();
    }

    Ok(())
}

async fn connection_writer_loop(
    messages: &mut Receiver<String>,
    stream: Arc<TcpStream>,
    shutdown: Receiver<Void>,
) -> Result<()> {
    let mut stream = &*stream;
    let mut messages = messages.fuse();
    let mut shutdown = shutdown.fuse();
    loop {
        select! {
            msg = messages.next().fuse() => match msg {
                Some(msg) => stream.write_all(msg.as_bytes()).await?,
                None => break,
            },
            void = shutdown.next().fuse() => match void {
                Some(void) => match void {},
                None => break,
            }
        }
    }
    Ok(())
}


async fn broker_loop(events: Receiver<Event>) {
    let (disconnect_sender, mut disconnect_receiver) = // 1
        mpsc::unbounded::<(String, Receiver<String>)>();
    let mut peers: HashMap<String, Sender<String>> = HashMap::new();
    let mut events = events.fuse();
    loop {
        let event = select! {
            event = events.next().fuse() => match event {
                None => break, // 2
                Some(event) => event,
            },
            disconnect = disconnect_receiver.next().fuse() => {
                let (name, _pending_messages) = disconnect.unwrap(); // 3
                assert!(peers.remove(&name).is_some());
                continue;
            },
        };
        match event {
            Event::Message { from, to, msg } => {
                for addr in to {
                    if let Some(peer) = peers.get_mut(&addr) {
                        let msg = format!("from {}: {}\n", from, msg);
                        peer.send(msg).await
                            .unwrap() // 6
                    }
                }
            }
            Event::NewPeer { name, stream, shutdown } => {
                match peers.entry(name.clone()) {
                    Entry::Occupied(..) => (),
                    Entry::Vacant(entry) => {
                        let (client_sender, mut client_receiver) = mpsc::unbounded();
                        entry.insert(client_sender);
                        let mut disconnect_sender = disconnect_sender.clone();
                        spawn_and_log_error(async move {
                            let res = connection_writer_loop(&mut client_receiver, stream, shutdown).await;
                            disconnect_sender.send((name, client_receiver)).await // 4
                                .unwrap();
                            res
                        });
                    }
                }
            }
        }
    }
    drop(peers); // 5
    drop(disconnect_sender); // 6
    while let Some((_name, _pending_messages)) = disconnect_receiver.next().await {
    }
}

fn spawn_and_log_error<F>(fut: F) -> task::JoinHandle<()>
where
    F: Future<Output = Result<()>> + Send + 'static,
{ // 스레드를 만들고 에러시 로그를 남기고 죽는다.
    task::spawn(async move {
        if let Err(e) = fut.await {
            eprintln!("{}", e)
        }
    })
}
```

