// We can use Result<T, E> to handle Recoverable errors and we cannot just crash our code

// fn divide(x:i32, y:i32) -> Result<i32, String> {
//     if y==0 {
//         return Err(String::from("You cannot divide by zero"));
//     }
//     return Ok(x/y);
// }

// fn main() {

//     let res= match divide(1, 0) {
//         Ok(num) => num,
//         Err(error) => {
//             println!("{error}");
//             -1
//         }
//     };

//     println!("res, {:?}",res);

// }

// use std::{fs::File, io::ErrorKind};


// fn main() {
//     let file = File::open("hello.txt");

//     let greeting_file = match file {
//         Ok(file) => file,
//         Err(error) => match error.kind() {
//             ErrorKind::NotFound => match File::create("hello.txt") {
//                 Ok(file) => file,
//                 Err(error) => panic!("Problem creating the file: {error:?}"),
//             },
//             _other => panic!("Problem opening the file: {error:?}")
//         }
//     };

//     // by this we save our program from crashing 
//     // we are opening the file if the file is not found then we are creating it

//     println!("{greeting_file:?}");
// }


use std::fs::File;
use std::io::{self, Read};

// Note: we can only use ? if the return type is Result<T, E>

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn main() {
    let res= read_username_from_file();

    println!("{res:?}");
}