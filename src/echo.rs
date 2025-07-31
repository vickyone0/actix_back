use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt,AsyncWriteExt};

pub async fn async_network_io() -> Result<(), Box<dyn std::error::Error>> {

    let listener = TcpListener::bind("127.0.0.1:8081").await?;
    println!("Listening on 127.0.0.1:8081");

    loop{
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buffer = [0;1024];
            loop {
                let n = socket.read(&mut buffer).await.unwrap();
                if n == 0 {
                    break;
                }

                socket.write_all(&buffer[..n]).await.unwrap();
            }
        });
    }

}
