use local_ip_address::list_afinet_netifas;
use local_ip_address::local_ip;

// Get the local IP address of system
pub fn get_local_ip() {
    let ip = local_ip().unwrap();
    println!("Local IP address: {ip:?}");
}

// Retrieve all the available network interfaces from both, the AF_INET and the AF_INET6 family
pub fn list_netifas() {
    let netifas = list_afinet_netifas().unwrap();
    for (name, ip) in netifas.iter() {
        println!("{name}:\t{ip:?}");
    }
}
