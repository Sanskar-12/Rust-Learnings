// difference between stack and heap

// 1. Stack can store the known length data -------- Heap can store the unknown length data
// 2. Accessing the data from the Stack is faster ----- Accessing the data from the heap can be slower because first we have to find the pointer inside the stack and then find the reference of the pointer in the heap 
// 3. Data is stored in the stack at the compile time ----- Data is stored in the heap at the run time
// 4. Data size cant be increased or decreased in the stack ------  Data size can be increased or decreased in the heap
 
fn main() {
    let s="Hello world"; // this string has a known length so this will be stored in the stack and we cannot append more string in it.

    let mut str=String::from("Hello world"); // this is stored inside the heap because we can append the strings and change the size of the string.
    str.push_str(" 2.0");

    println!("Str is {str}");

}

// ownership concept if the owner of a memory goes out of scope then rust frees the memory
