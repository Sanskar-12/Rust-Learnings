fn main() {
    // let
    let mut age=23;
    println!("Age, {age}");
    
    // we cannot mutate the immutable variable in rust
    // age=34;
    // println!("Age, {age}");

    // but we can mutate the variables by mut keyword
    age=34;
    println!("Age,{age}");

    // -------------------------------------------//

    // const
    // we cannot mutate the const in any means, and also we have to define type while using const
    const PI:f64=3.14;

    println!("PI, {PI}");

    const ONEHOUR: u64=60*60;

    let timer=ONEHOUR*3;

    println!("{timer}");

    // shadowing concept - in this concept we declare the same variable name and one variable is ignored by the other variable
    let apples=20;  // this var is shadowed by the below var

    let apples=30;

    println!("apples {apples}")
}
