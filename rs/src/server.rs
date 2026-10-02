use super::resp::Resp;
use anyhow;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

pub async fn start(addr: String) -> anyhow::Result<()> {
    let listener = TcpListener::bind(addr).await?;
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

async fn handle_client(mut stream: TcpStream) -> anyhow::Result<()> {
    let mut buf = [0_u8; 512];

    loop {
        match stream.read(&mut buf).await {
            Ok(size) if size != 0 => {
                println!("Received: {}", str::from_utf8(&buf.to_vec()).unwrap());

                let response = Resp::simple_string("PONG");

                if let Err(e) = stream.write_all(response.to_string().as_bytes()).await {
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
