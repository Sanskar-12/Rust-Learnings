use std::collections::HashMap;

fn main() {
    // ---------------------creating the hashmap----------------------//
    let mut scores=HashMap::new();

    scores.insert(String::from("blueteam"), 20);
    scores.insert(String::from("redteam"), 30);
    scores.insert(String::from("yellowteam"), 30);

    // println!("Hashmap is {:?}",scores);
    
    // ----------------------accessing the value----------------------//
    // let value=scores.get(&String::from("blueteam")).unwrap_or(&0);

    // println!("score of blue team is {value}");

    // ------------itrating the hashmap-----------------//
    // for (key,value) in &scores {
    //     println!("{:?} -> {:?}",key,value);
    // }

    //----------- we can overwrite the value of the key----------------//

    // scores.insert(String::from("blueteam"), 100);

    // for (key,value) in &scores {
    //     println!("{:?} -> {:?}",key,value);
    // }

    // ---------- we can add the key value if the key is not present------------//
    scores.entry(String::from("greyteam")).or_insert(170);

    for (key,value) in &scores {
        println!("{:?} -> {:?}",key,value);
    }

    // ----------------if we have to count the no of words in text---------------//

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}
