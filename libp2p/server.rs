use std::net::{TcpStream, TcpListener};
use std::io::{Read, Write};
use std::thread;

struct MyConnection {
    stream: TcpStream
}

impl MyConnection {
    fn new(stream: TcpStream) -> MyConnection {
        MyConnection { stream: stream }
    }

    fn send(&mut self, message: &str) {
        self.stream.write(message.as_bytes()).unwrap();
    }

    fn receive(&mut self, buffer: &mut [u8]) {
        self.stream.read(buffer).unwrap();
    }
}

struct MyListener {
    listener: TcpListener
}

impl MyListener {
    fn new(addr: &str) -> MyListener {
        let listener = TcpListener::bind(addr).unwrap();
        MyListener { listener: listener }
    }

    fn accept(&self) -> MyConnection {
        let (stream, _) = self.listener.accept().unwrap();
        MyConnection::new(stream)
    }
}

fn main() {
    let listener = MyListener::new("127.0.0.1:8080");
    println!("Listening on {}", "127.0.0.1:8080");

    for stream in listener.listener.incoming() {
        let conn = MyConnection::new(stream.unwrap());

        thread::spawn(move || {
            let mut buf = [0; 1024];
            conn.receive(&mut buf);
            let message = String::from_utf8_lossy(&buf[..]);
            println!("Received message: {}", message);
            conn.send("Hello from the server!");
        });
    }
}