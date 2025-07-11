use tokio::io::{self, AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio::net::{TcpListener};

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    println!("listening on port {}", listener.local_addr()?.port());

    loop {
        let (mut socket, addr) = listener.accept().await?;

        println!("connection from {addr:?}");

        tokio::spawn(async move {
            if let Err(e) = handle_request(&mut socket).await {
                eprintln!("Connection error: {e}");
            }
        });
    }
}

async fn handle_request(socket: &mut (impl AsyncWrite + AsyncRead + Unpin)) -> io::Result<()> {
    socket.write_all(b"Who are you?\n").await?;

    let mut buf = vec![0; 1024];
    let name_size = socket.read(&mut buf).await?;
    let name = std::str::from_utf8(&buf[..name_size]).unwrap().trim();
    let reply = format!("Thanks for dialing in, {name}!\n");
    socket.write_all(reply.as_bytes()).await?;
    Ok(())
}