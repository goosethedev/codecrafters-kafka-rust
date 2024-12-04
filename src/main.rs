use std::io::Cursor;

use anyhow::Result;
use tokio::net::TcpListener;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:9092").await?;

    loop {
        let (stream, _) = listener.accept().await?;

        tokio::spawn(async move {
            println!("accepted new connection");
            handle_connection(stream).await.expect("connection error");
        });
    }
}

async fn handle_connection(mut stream: TcpStream) -> Result<()> {
    // Read message_size
    let mut len = [0; 4];
    stream.read_exact(&mut len).await?;
    let len = i32::from_be_bytes(len);

    // Read message content
    let mut msg = vec![0; len as usize];
    stream.read_exact(&mut msg).await?;

    // Use a buffered cursor to read parts of the message
    let mut msg = Cursor::new(msg);
    msg.read_i16().await?; // request api key
    msg.read_i16().await?; // request api version
    let correlation_id = msg.read_i32().await?;

    stream.write_all(&[0; 4]).await?;
    stream.write_all(&correlation_id.to_be_bytes()).await?;

    Ok(())
}
