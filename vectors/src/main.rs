// vector is array which is dynamic

fn main() {
    // let mut v = Vec::new();
    let mut v = vec![1,2,3]; // we can also do this if we have to give default value

    // v.push(1);
    // v.push(2);
    // v.push(3);

    // println!("2nd index is {}",&v[2]);

    let does_not_exist=v.get(1); // it does not exist so we have to do match operation and catch the error else the code will panic

    let tenth_value=match  does_not_exist  {
        Some(value) => println!("10th val is {}",value), // if exist it will print this
        None => println!("Index does not exist")  // if doesnt exist it will print this
    };




    println!("vec is {:?}",v);
}
