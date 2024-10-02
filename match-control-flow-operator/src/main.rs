// ----------------------match operator matches all the condition given inside it and then returns the matched value -----------//
// Its like the switch case in javascript 

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
}


enum Coins {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coins:Coins) -> u8 { // we have to cover all the cases of the Coins enum inside match operator if not it'll give error
    return match coins {
        Coins::Nickel => 1, // if the returning value is not on one line 
        Coins::Dime => { // if multiple then we have to use curly brackets
            println!("Coin is Dime");
            2
        },
        Coins::Penny => 3,
        Coins::Quarter(state) => {
            println!("The value of quarter in the {state:?}");
            4
        }
    };
}

fn add(num:i32, num2:Option<i32>) -> Option<i32> {
    match num2 {
        Some(i) => Some(num+i),
        None => Some(num)
    }
}

fn match_dice(dice_number:i32)  {
    match dice_number {
        3 => println!("This is 3"),
        4 => println!("This is 4"), // you can handle these cases and for rest you can use other
        other => println!("this is other {}",other) // other is used when you dont want to handle all the cases
        // we can write other or just an underscore
    }
}

fn main() {
    let val = value_in_cents(Coins::Quarter(UsState::Alabama));
    let val2 = add(3, Some(3));
    let val3 = add(3, None);
    match_dice(6);
    println!("{val},{},{}",val2.unwrap(),val3.unwrap()); // if we have to convert the Option<i32> to i32 is to unwrap the val
}
