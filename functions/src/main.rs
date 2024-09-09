// function declaration
fn myfunction() {
    println!("Hello from myfunction!")
}

// parameterized function
fn myfunction2(x:i32, y:bool) {
    println!("X:{x} and Y:{y}");

    // let x=3; // this is a statement because it doesnt return any value
    // let y=x+10; // this x+10 is an expression it returns the value

    // let x=(let y=10) ----> we cant do that because statement doesnt return any value
}

fn myfunction3() {
    let y={
        let x=3;
        x+1 // this is and expression which will return the value because we have not semicolon ahead .. so this means if we dont put semicolon so it becomes an expression
    };

    println!("{y}"); // y value will be 4
}

fn add(x:i32, y:i32) -> i32 {
    return x+y;
}

fn main() {
    println!("Hello, world!");

    myfunction(); // function calling
    myfunction2(2, true);
    myfunction3();
    let res=add(2, 3);
    println!("{res}");
}


