use std::net::TcpStream;
use std::collections::VecDeque;
use std::net::TcpListener;
use std::net::Shutdown;
use std::io::Read;
use std::str;

#[derive(Debug)]
enum Event {
    Accept(TcpStream),
    Read(TcpStream),
}

#[derive(Debug)]
pub struct EventQueue {
    q:VecDeque<Event>
}

impl EventQueue {

    pub fn new(initial_capacity:usize) -> Self {
        EventQueue { q : VecDeque::with_capacity(initial_capacity) } 
    }

    pub fn push_accept(&mut self, listener:&TcpListener) {

        let result = listener.accept();
        match result {
            Ok((sock, _addr)) => {
                self.q.push_back( Event::Accept(sock) );
            },
            Err(e) => {
                println!("client error: {e:?}");
            }
        }
        
    }

    pub fn push_read(&mut self, stream:TcpStream) {

        self.q.push_back( Event::Read(stream) );

    }

}

#[derive(Debug)]
pub struct EventDispatcher {

}

impl EventDispatcher {
    pub fn new() -> Self {
        EventDispatcher {} 
    }

    pub fn dispatch(&self, eq:&mut EventQueue) {
        let ev = eq.q.pop_front().unwrap();
        match ev {
            Event::Accept(stream) => {
                {
                    let addr = stream.peer_addr().unwrap();
                    println!("Accepting remote stream from: {:?}", addr);
                }
                eq.push_read( stream );
            },
            Event::Read(mut stream) => {
                let mut buff:[u8; 256] = [0; 256];
                let size = stream.read( & mut buff ).unwrap();
                println!("Bytes read from {:?}: {size}", size);
                let _ = stream.shutdown(Shutdown::Both);
            },
            _ => {}
        }
    }
}

