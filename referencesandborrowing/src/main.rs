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
// fn main() {
//     let mut s=String::from("Hello world");
//     let len = calculate_length(&mut s); 
//     println!("length of {s} is {len}")
// }

// fn calculate_length(s1:&mut String) -> usize { // here s1 is a mutable reference variable which is pointing to s and s is pointing to the heap 
//     s1.push_str("Sanskar"); // we can mutate the value because i have made the reference as mutable
//     let res = s1.len();
//     res
// }


// ---------------------------------cannot have nultiple mutable refereces---------------//
// fn main() {
    // let mut s=String::from("Hello");
    // let s1=&mut s;
    // let s2=&mut s; // it will give error because you cannot have multiple mutable references in the same scope

    // s2.push_str("Hello");

    // println!("{s1}")

    // {
    //     let r1=&mut s; // this will not give error because both have different scope in which they are defined
    // }

    // let r2=&mut s;
    // r2.push_str("Hello");

    // println!("{r2}");
// }

// ---------------------------dangling reference-------------//

fn main() {
//     let reference_to_nothing = dangle();
// }

// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s // this will give error because we cannot return the reference of the varaible
}
