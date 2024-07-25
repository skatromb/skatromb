
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn route(ip: IpAddr) {
    println!("{:?}", ip);
    match ip {
        IpAddr::V4(ip1, ip2, ip3, ip4) => {
            println!("Address: {}.{}.{}.{}", ip1, ip2, ip3, ip4)
        },
        IpAddr::V6(address) => println!("Address: {}", address),
    }

}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    
    let loopback = IpAddr::V6(String::from("::1"));
    
    route(home);
    route(loopback);
}
