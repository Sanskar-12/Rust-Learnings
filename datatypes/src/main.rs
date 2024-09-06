fn main() {
    // we have to specify the datatype before parsing the value, in which datatype we have to parse
    let guess:u32="43".parse().expect("Not a number");
    println!("{guess}");

    // signed means the number has either positive sign or negative sign, for eg i32
    let signednegative:i32=-45;
    let signedpositive:i32=45;

    // unsigned means the number has only positive sign, for eg u32
    let unsigned:u32=45;
}
