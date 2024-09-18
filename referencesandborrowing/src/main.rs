// in ownership we were coping the value or changing the owner of the value but in reference we are passing a pointer to the value but the value is not copied into it

// fn main() {
//     let s=String::from("Hello world");
//     let len = calculate_length(&s); 
//     println!("length of {s} is {len}")
// }

// fn calculate_length(s1:&String) -> usize { // here s1 is a reference variable which is pointing to s and s is pointing to the heap 
//     // s1.push_str("Sanskar"); // this will give error because i have borrowed the value of s so you cant mutate or change the value of the variable because you are not the owner of the value
//     let res = s1.len();
//     res
// }

// ----------------------------------------mutable references ----------------------//
fn main() {
    let mut s=String::from("Hello world");
    let len = calculate_length(&mut s); 
    println!("length of {s} is {len}")
}

fn calculate_length(s1:&mut String) -> usize { // here s1 is a mutable reference variable which is pointing to s and s is pointing to the heap 
    s1.push_str("Sanskar"); // we can mutate the value because i have made the reference as mutable
    let res = s1.len();
    res
}
