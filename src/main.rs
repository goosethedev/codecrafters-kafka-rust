use std::{io::Write, net::TcpListener};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9092").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");
                let buf = [0, 0, 0, 0, 0, 0, 0, 7];
                stream.write_all(&buf).expect("Error sending response");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
