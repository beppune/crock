
use std::io::Write;
use std::io::Read;
use std::net::TcpListener;
use std::net::Shutdown;
use std::str;

mod request;
use self::request::HttpRequest;

fn main() {
    
    const HOST: &str = "127.0.0.1";
    const PORT: &str = "8000";

    let endpoint: String = HOST.to_owned() + ":" + PORT;

    let listener = TcpListener::bind(endpoint.clone()).unwrap();

    println!("Listening on {}", endpoint);

    for stream in listener.incoming() {

        let mut buffer: [u8; 256] = [0; 256];

        let mut peer = stream.unwrap();

        let peer_addr = peer.peer_addr().unwrap();

        println!("Accepting connection from: {}", peer_addr);

        let s: usize = peer.read(&mut buffer).unwrap();

        print!("Bytes red from {}: {}: ", peer_addr, s);

        let message = str::from_utf8(&buffer[..s]).unwrap();
    
        let r = HttpRequest::deserialize(&message).expect("Nothing found");

        if let HttpRequest::GET(some) = r {
            println!("From client: {}", some);
        }

        let _ = peer.write( "Hello\n".as_bytes() );

        peer.shutdown(Shutdown::Both).expect("Ciaone");

    }

}
