use anyhow;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

async fn handle_client(mut stream: TcpStream) -> anyhow::Result<()> {
    let mut buf = [0_u8; 512];

    loop {
        match stream.read(&mut buf).await {
            Ok(size) if size != 0 => {
                println!("Received: {}", str::from_utf8(&buf.to_vec()).unwrap());

                let response = "+PONG\r\n";

                if let Err(e) = stream.write_all(response.as_bytes()).await {
                    eprintln!("Error writing to socket: {e}");
                }
            }
            Ok(_) => {
                println!("Connection closed");
                break;
            }
            Err(e) => {
                eprintln!("Error: {e}");
                break;
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379").await?;
    println!("Listening on {}", listener.local_addr()?);

    loop {
        let (stream, peer) = listener.accept().await?;
        println!("Accepted: {peer}");

        tokio::spawn(async move {
            if let Err(err) = handle_client(stream).await {
                eprintln!("Connection error: {err}")
            }
        });
    }
}
