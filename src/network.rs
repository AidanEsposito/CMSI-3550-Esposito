use async_std::net::{TcpListener, TcpStream};
use futures::stream::StreamExt;
use tokio::time::{sleep, Duration};

pub async fn start_network_listener(addr: &str) -> std::io::Result<()>{
    let listener= TcpListener::bind(addr).await?;

    let mut incoming = listener.incoming();
    while let Some(stream) = incoming.next().await{
        match stream {
            Ok(stream) => {
                handle_connection("Accepted", stream).await;
            }
            Err(e) => eprintln!("Accept failed: {:?}", e),
        }
    }
    Ok(())
}

pub async fn start_network_connector(addr: &str) -> std::io::Result<()>{
    loop{
        let stream = TcpStream::connect(addr).await;
        match stream{
            Ok(stream) => {
                handle_connection("Connected", stream).await;
            }
        Err(e) => eprintln!("Connect failed: {:?}", e),
        }
        sleep(Duration::from_millis(1000)).await;
    }
    Ok(())
}


async fn handle_connection(sender: &str, stream: TcpStream){
    println!("{} from: {}", sender, stream.peer_addr().unwrap())
}