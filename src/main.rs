//Aidan Esposito

//Impement p2p network
//Someone is listening
//Someone is connecting

//p2p network is run entirely locally

//cargo run -- localhost:5555 localhost:8888
//cargo run -- localhost:8888 localhost:5555

mod network;
use std::env::args;

#[tokio::main]

async fn main() {

    let my_args: Vec<String> = args().collect();
    let local_addr = my_args[1].to_string();
    let remote_addr = my_args[2].to_string();

    //start network listener
    let listener_task = tokio::spawn(async move {
        network::start_network_listener(&local_addr).await.unwrap();
    });

    //start the network connector
    let connector_task = tokio::spawn(async move {
        network::start_network_connector(&remote_addr).await.unwrap();
    });

    //Wait for both tasks
    let _ = tokio::try_join!(listener_task, connector_task);
}