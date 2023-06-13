use std::net::{TcpListener, TcpStream};

pub fn connect() {
    let listener = TcpListener::bind("127.0.0.1:8664").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_conn(stream);
    }
}

fn handle_conn(stream: TcpStream) {
    println!("jaja");
}
