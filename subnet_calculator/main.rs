//Aidan Esposito

use std::net::Ipv4Addr;

fn main() {
    // Find target IP and Subnet Notation
    let target_ip: Ipv4Addr = "157.242.0.0".parse().unwrap();
    let cidr_notation = 16;

    // Subnet mask
    let subnet_mask: u32 = 0xffffffff << (32 - cidr_notation);
    let subnet_mask = Ipv4Addr::from(subnet_mask.to_be());

    // Subnet values
    let subnet_ip = target_ip;
    let subnet_broadcast_ip = subnet_ip | !subnet_mask;
    let subnet_host_min = u32::from(subnet_ip) + 1;
    let subnet_host_max = u32::from(subnet_broadcast_ip) - 1;

    // Next Network
    let next_network_ip = u32::from(subnet_broadcast_ip) + 1;

    // IP Address #
    let num_addresses = 2u32.pow(32 - cidr_notation);

    // Output subnet values
    println!("Network ID: {}", subnet_ip);
    println!("Broadcast IP: {}", subnet_broadcast_ip);
    println!("First Host: {}", Ipv4Addr::from(subnet_host_min.to_be()));
    println!("Last Host: {}", Ipv4Addr::from(subnet_host_max.to_be()));
    println!("Next Network: {}", Ipv4Addr::from(next_network_ip.to_be()));
    println!("Number of IP addresses: {}", num_addresses);
    println!("CIDR/Subnet Mask: /{}", cidr_notation);
}


