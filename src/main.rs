
use std::io::Write;
use std::io::Read;
use std::net::TcpListener;
use std::net::Shutdown;
use std::str;

mod events;

use self::events::EventQueue;
use self::events::EventDispatcher;

fn main() {

    const HOST: &str = "127.0.0.1";
    const PORT: &str = "8000";

    let endpoint: String = HOST.to_owned() + ":" + PORT;

    let listener = TcpListener::bind(endpoint.clone()).unwrap();

    println!("Listening on {}", endpoint);

    let mut EQ = EventQueue::new(listener, 10);

    EQ.push_accept();

    let mut ED = EventDispatcher::new(EQ);

    loop {
        ED.dispatch();
    }
}
