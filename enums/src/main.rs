// #[derive(Debug)]
// enum IpAddrKind {
//     V4,
//     V6
// }

// struct IpAddress {
//     address:String,
//     kind:IpAddrKind
// }

// fn route(addr:IpAddress) {
//     println!("the {} is of this {:?} kind",addr.address,addr.kind);
// }

// fn main() {
//     let google_ip=IpAddress {
//         address:String::from("1.0.0.0"),
//         kind:IpAddrKind::V4
//     };

//     route(google_ip);
// }

// ------------------ we can directly attach the data with the enum------------//

#[derive(Debug)]
enum IpAddrKind {
    V4(String),
    V6(u8,u8,u8,u8) // this is tuple
}

fn route(ip:IpAddrKind) {
    println!("routing to ip address {ip:?}")
}

fn main() {
    route(IpAddrKind::V4(String::from("1.0.0.0")));
    route(IpAddrKind::V6(127,0,0,1));
}


