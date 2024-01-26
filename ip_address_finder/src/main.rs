//Aidan Esposito

use local_ip_address::local_ip;
fn main() {
    let my_ip = local_ip().unwrap();
    println!("Your IP address is: {:?}", my_ip);
}