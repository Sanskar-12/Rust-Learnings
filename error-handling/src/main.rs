// Now there are two types of errors 
// 1. Recoverable errors by which your code can recover by giving some error message and your program will not crash eg:- file not found
// 2. UnRecoverable errors by which your code can crash eg:- accessing undefined element in the array

// So in Unrecoverable errors we you use panic! by which my program will empty the memory and stack used by code and exit 


fn main() {
    println!("Hello, world!");

    // panic!("This is a panic line"); // this will panic

    // one more example

    // let arr=[1,2,3,4,5];

    // println!("Give me ele no 10 {}",arr[10]); // this also will panic 

}
