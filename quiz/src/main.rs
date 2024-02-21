use std::env::args;

fn main(target_string: String){
    let parts = target_string.split(":");
    for parts in parts{
        println!("Subnet part: {part}");
    }

    let [target_string, cidr]: [&str; 4] = target_subnet.split(":").collect::<Vec<&str>>().try_into().unwrap_or_default();

    println!("Parsed String: {target_string}");
}