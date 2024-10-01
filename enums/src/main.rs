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

// #[derive(Debug)]
// enum IpAddrKind {
//     V4(String),
//     V6(u8,u8,u8,u8) // this is tuple
// }

// fn route(ip:IpAddrKind) {
//     println!("routing to ip address {ip:?}")
// }

// fn main() {
//     route(IpAddrKind::V4(String::from("1.0.0.0")));
//     route(IpAddrKind::V6(127,0,0,1));
// }

// ---------Option enum---------------------------//

// Rust doesnt have null value so Option enum has made which has None and Some value

// enum Option<T> { // this way Option enum is made
//     None,
//     Some(T)
// }

fn main() {
    let enum1: Option<i32>=Option::Some(1); // if the value contains then we use Some
    let enum2: Option<i32>=Option::None; // if the value doesnt contains or it is Null then we use None

    let x=10;

    let res=x+enum1.unwrap_or(0); // we can unwrap the enum1 to take the number from the Option::Some()
    println!("{res}");
}

