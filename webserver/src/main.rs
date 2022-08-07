use std::net::TcpListener;

fn main() {
    let tcplistener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in tcplistener.incoming() {
        let stream = stream.unwrap();

        println!("Connection Received!!!");
    }
}
