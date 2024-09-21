// fn main() {
//     let mut s=String::from("Hello world");

//     let res=first_word(&s);

//     s.clear(); // this is will create a bug because we are giving the result of an empty string as 5 so we have to keep track of the s String so for this we have slices
//     println!("The Length of the first word of the {s} is {res}");
// }

// fn first_word(s:&String) -> usize {
//     let bytes=s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item==b' ' {
//             return i;
//         }
//     }

//     return s.len();
// }

// --------------------------Slices-------------------//
// Slices are references to a part of a string so we can keep track of all the indices
// fn main() {
//     let s=String::from("Hello world");

//     let first_word=&s[0..5];
//     let second_word=&s[6..11];

//     // s.clear(); // jab tak first_word jo ki reference hai s uska jab tak scope jinda hai tab tak s ko mutate nhi karsakte 

//     println!("First word of {s} is {first_word}");
//     println!("Second word of {s} is {second_word}");
// }

// ------------------------so coming to our previous code-------------//

fn main() {
    let s=String::from("Hello world");

    let res=first_word(&s); // res is slice of s

    // s.clear(); // ab res is the slice so jab tak slice ka scope valid hai uske scope ke andar tu s ko change bhi nhi karsakta
    println!("The Length of the first word of the {s} is {res}");
}

fn first_word(s:&str) -> &str { // str will handle the strings in the heap as well as strings in the stack written by senior coders in rust
    let bytes=s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item==b' ' {
            return &s[0..i];
        }
    }

    return &s[..];
}