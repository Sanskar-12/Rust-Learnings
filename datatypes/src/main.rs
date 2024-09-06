fn main() {
    // we have to specify the datatype before parsing the value, in which datatype we have to parse
    let guess:u32="43".parse().expect("Not a number");
    println!("{guess}");

    // -------------------------------------------------//
    // integer data type (i) and (u)
    // signed means the number has either positive sign or negative sign, for eg i32
    let signednegative:i32=-45;
    let signedpositive:i32=45;

    // unsigned means the number has only positive sign, for eg u32
    let unsigned:u32=45;

    // ------------------------------------------//

    // Integer overflow 
    // Suppose we have a variable type u8 which has a range of 0 to 255 and if we try to give value 256 to the var then it will cause integer overflow
    // let a:u8=256; --> this will cause integer overflow
    
    // so in debug mode if we run this code then it will perform integer overflow checks but if we run this code in release mode then it will perform two complement wrapping 
    // what is twos complement wrapping
    // u8 range is 0 to 255
    // twos complement converts 256 to 0 , 257 to 1 ,258 to 2 and so on, so it creates a cycle if  it gets the value greater than 255 then it starts to convert them from 0
    let a:u8=random()+56; // so this will not panic it will give 0 because 256 is 0 according to twos complement wrapping

    println!("the value of a, {a}");

    // -----------------------------------------------//
    // floating point datatype (f)
    let b: f64=45.0;
    let c: f32=45.0;

    // ----------------------------------------------//
    // boolean datatype (bool)
    let t:bool=true;
    let f:bool=false;

    // ---------------------------------------------//
    // string and char data type
    let d: char='p';
    let name: &str="sanskar";

    // -------------------------------------------------//
    // compound data type --> array and tuple --> they can hold muliple datatypes at once
    // tuple --> in this we can store diff types of data types at once
    let sanskar: (&str, i32, bool)=("sanskar",65,true);

    // accessing there are two ways
    // 1st way
    let name: &str=sanskar.0;
    let weight: i32=sanskar.1;
    let male: bool=sanskar.2;

    // 2nd way is destructuring
    let (name1,weight1,male1)=sanskar;

    // arrays --> in this we can store same type of elements
    let arr: [i32; 3]=[23,34,44];

    let arr2=[10;5]; // --> [10,10,10,10,10]
}

fn random()->u8 {
    return 200;
}
