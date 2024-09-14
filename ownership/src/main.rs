// difference between stack and heap

// 1. Stack can store the known length data -------- Heap can store the unknown length data
// 2. Accessing the data from the Stack is faster ----- Accessing the data from the heap can be slower because first we have to find the pointer inside the stack and then find the reference of the pointer in the heap 
// 3. Data is stored in the stack at the compile time ----- Data is stored in the heap at the run time
// 4. Data size cant be increased or decreased in the stack ------  Data size can be increased or decreased in the heap

fn takesownership(s:String) {
    println!("{s}");
}
 
fn main() {
    // let s="Hello world"; // this string has a known length so this will be stored in the stack and we cannot append more string in it.

    // let mut str=String::from("Hello world"); // this is stored inside the heap because we can append the strings and change the size of the string.
    // str.push_str(" 2.0");

    // println!("Str is {str}");

    // -----------------------------------------------//

    // this two variables will be stored inside the stack because they have known length
    // let mut x = 5; 
    // let y = x; // this is pass by value means only value is copied inside y

    // x=6;
    // println!("{y}"); // this will give us y=5

    // --------------------------------------//
    
    // let str=String::from("Hello"); // this is pass by reference and both the variables will point to same memory location inside the heap

    // let y=str;

    // println!("{str}"); // this will give error because when i have assigned the str to y then rust will free str and str does not exist in the memory because if rust doesnt free the str then it will cause double free error

    // -------------------------------------------//

    // let str=String::from("Hello"); // this is pass by reference and both the variables will point to same memory location inside the heap

    // let y=str.clone(); // this will make a clone of the str and only y will be pointing to that so you can assume it as pass by value

    // println!("{str}"); // this will not give error because now y has another string same as str

    // -----------------------------------//
    let str=String::from("Hello"); // passing the value of String datatype inside a function also changes the ownership

    takesownership(str);

    // println!("{str}"); // thats why its giving error because the ownership changed when we pass the value inside the function

}



// ownership concept if the owner of a memory goes out of scope then rust frees the memory
