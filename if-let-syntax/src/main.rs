fn main() {
    let config_max = Some(3u8);
    
    // match config_max {
    //     Some(max) => println!("The maximum is configured to be {max}"),
    //     _ => (), // we have to write this extra line in match operator
    // }

    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}")
    }
}

// so in if let syntax we dont have to consider all the cases unlike the match operator