// vector is array which is dynamic
#[derive(Debug)]
enum Spreadsheet {
    Int(i32),
    Float(f64),
    Text(String)
}

fn main() {
    // let mut v = Vec::new();
    // let mut v = vec![1,2,3]; // we can also do this if we have to give default value

    // v.push(1);
    // v.push(2);
    // v.push(3);

    // println!("2nd index is {}",&v[2]);

    // let does_not_exist=v.get(1); // it does not exist so we have to do match operation and catch the error else the code will panic

    // let tenth_value=match  does_not_exist  {
    //     Some(value) => println!("10th val is {}",value), // if exist it will print this
    //     None => println!("Index does not exist")  // if doesnt exist it will print this
    // };

    // println!("vec is {:?}",v);
//-------------------------------------------------------------------------------------------//

    // let v = vec![1, 2, 3, 4, 5];

    // let first = &v[0];

    // v.push(1);  // this will give error because whenever we push the element in the vector so rust will make a new vector and put all the elements in that new vector and thats why the memory location changes so it will give error


//------------------------------- Iterate in the vector----------------------------------------------------------//

    // let v = vec![1,2,3,4,5];

    // for i in v {
    //     println!("i is {}",i);
    // }

    // println!("{:?}",v);  // this will give error because we have given ownership of v to i variable so if dont have to give ownership we have give & reference

    // let v = vec![1,2,3,4,5];

    // for i in &v {
    //     println!("i is {}",i);
    // }

    // println!("{:?}",v); 

    // // ---------------------dereferencing------------------------------------//

    // let mut v = vec![1,2,3,4,5];

    // for i in &mut v {
    //     println!("i is {}",i);
    //     *i = *i * 2; // this is called dereferencing means we have changed the reference value to dereference so the vector can be changed
    // }

    // println!("{:?}",v); 

    // ----------------------------------enums with vectors-------------------//
    // so as we know we cannot have different type of values inside the vectors so we can use enums and vectors

    let v = vec![
        Spreadsheet::Int(30),
        Spreadsheet::Float(3.0),
        Spreadsheet::Text(String::from("Nice"))
    ];

    println!("value or v is {:?}",v);

    match v.get(0) {
        Some(Spreadsheet::Int(value)) => println!("This is the value {}",value), // match if the value passed is int
        Some(other) => println!("Some other value"), // match if the value passed insted of int
        None => println!("No value") // match if the value is not there
    }


}
