use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number=rand::thread_rng().gen_range(1..=100);

    println!("Random Number is {secret_number}");

    println!("Please input your guess");


    let mut guess=String::new();

    io::stdin().read_line(&mut guess).expect("Error");

    let guess:u32=guess.trim().parse().expect("Failed to parse"); //shadowing the variable

    match guess.cmp(&secret_number) {
        Ordering::Equal => println!("You Won!"),
        Ordering::Greater => println!("Number Guessed is Greater, Please choose Lesser Number!"),
        Ordering::Less => println!("Number Guessed is Lesser, Please choose Greater Number!")
    }

}
