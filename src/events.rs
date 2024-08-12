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
    q:VecDeque<Event>,
    listener:TcpListener,
}

impl EventQueue {

    pub fn new(listener:TcpListener, capacity:usize) -> Self {
        EventQueue {
            q : VecDeque::with_capacity(capacity),
            listener,
        } 
    }

    pub fn push_accept(&mut self) {

        let result = self.listener.accept();
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
    eq:EventQueue,
}

impl EventDispatcher {
    pub fn new(eq:EventQueue) -> Self {
        EventDispatcher { eq } 
    }

    pub fn dispatch(&mut self) {
        let ev = self.eq.q.pop_front().unwrap();
        match ev {
            Event::Accept(stream) => {
                {
                    let addr = stream.peer_addr().unwrap();
                    println!("Accepting remote stream from: {:?}", addr);
                }
                self.eq.push_read( stream );
            },
            Event::Read(mut stream) => {
                let mut buff:[u8; 256] = [0; 256];
                let size = stream.read( & mut buff ).unwrap();
                let s = str::from_utf8( &buff ).unwrap();
                println!("Bytes read from {:?}: {size}. Content: {}", size, s);
                let _ = stream.shutdown(Shutdown::Both);

                self.eq.push_accept();
            },
        }
    }
}

