// UTF-8 is a character encoding standard that represents text in computers, where each character is encoded using 1 to 4 bytes. It is capable of encoding all possible characters in Unicode and is widely used due to its efficiency and compatibility with ASCII.
// unicode is a universal character encoding standard which understands all the languages in the world
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    // -------------------------------------we can push into the strings or character----------------------------------------//
    // let mut hello = String::from("नमस्ते");

    // hello.push_str(" in hindi"); // pushing string

    // hello.push('.'); // pushing character

    // println!("{hello}");

    //------------------------------ we can also concatenate the two strings------------------------------//
    // let mut s1 = String::from("Hello");
    // let s2 = String::from(" world");

    // let s3 = s1 + &s2; // we can not use the s1 after this line because we have given the ownership of s1

    // println!("{s3}");

    // s1.push('3');

    // ---------------------------------why we cant access the characters by index like other language in rust-----------------//

    // let hello = "Здравствуйте";  // This is an exception here 3 means Ze which is 2 bytes long and we cant access 2 bytes at same time
    // let answer = &hello[0]; // so this will give error

    // ------------------iterating on bytes, characters(scalars)--------------------//
    let hello = String::from("नमस्ते");

    // bytes
    for c in hello.as_bytes() {
        println!("c = {c}");
    }

    // scalar or characters
    for c in hello.chars() {
        println!("c = {c}");
    }

    // graphemes (it uses external crate which is unicode-segmentation)
    for c in hello.graphemes(true).collect::<Vec<&str>>() {
        println!("c = {c}");
    }
}
