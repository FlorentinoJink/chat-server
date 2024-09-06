use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;


#[tokio::main]
async fn main() -> anyhow::Result<()>{
    let server = TcpListener::bind("127.0.0.1:54321").await?;
    let (mut tcp,_) = server.accept().await?;
    let mut buffer = [0u8;16];

    loop {
        let n = tcp.read(&mut buffer).await?;
        if n == 0 {
            break;
        }
        let _ = tcp.write(&buffer[..n]).await?;
    }
    Ok(())
}
