use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    
    let secret_number=rand::thread_rng().gen_range(1..=100);
    println!("Random Number is {secret_number}");
    
    loop {
        println!("Guess the number!");

        println!("Please input your guess");


        let mut guess=String::new();

        io::stdin().read_line(&mut guess).expect("Error");

        let guess:u32=match guess.trim().parse() {
            Ok(num)=>num,
            Err(_)=>{
                println!("Please enter valid input");
                continue;
            },
        }; //shadowing the variable

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You Won!");
                break;
            },
            Ordering::Greater => println!("Number Guessed is Greater, Please choose Lesser Number!"),
            Ordering::Less => println!("Number Guessed is Lesser, Please choose Greater Number!")
        }
    }
}
