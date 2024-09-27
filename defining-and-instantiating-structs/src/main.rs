// fn area(length:u32, width:u32) -> u32 {
//     length*width
// }

// fn main() {
//     let l=30;
//     let b=40;
//     let res=area(l, b);   // this code is not clear and readable because we can mismatch the length and width 
//     println!("res is {res}");
// }

//--------------instead we can make tuple------------------//

// fn area(rect:(u32,u32)) -> u32 {
//     rect.0*rect.1
// }
// fn main() {
//     let rect=(30,40);
//     let res=area(rect);   // this code is somewhat clear and readable but when we pass the value inside the function we can mismatch the values because we dont know if the first one is length or second one is width
//     println!("res is {res}");
// }

// -------------------------the most optimal way to use struct--------------//
// struct Rectangle {
//     length:u32,
//     width:u32
// }

// fn area(rect:&Rectangle) -> u32 { // yaha pe we are taking the ownership of rect so we can take the instance by making it &
//     rect.length*rect.width
// }
// fn main() {
//     let rect= Rectangle {
//         length:30,
//         width:40
//     };
//     let res=area(&rect);  // so this is more clean and readable because we know that we are passing the length and width at the correct place
//     println!("res is {res}");
// }

//------------------Debug trait (:?) and (:#?) for pretty print -- new Display format-------//
#[derive(Debug)]
struct Rectangle {
    length:u32,
    width:u32
}

fn area(rect:&Rectangle) -> u32 { 
    rect.length*rect.width
}
fn main() {
    let rect= Rectangle {
        length:30,
        width:40
    };
    let res=area(&rect);  
    println!("res of {rect:#?} is {res}");
}
