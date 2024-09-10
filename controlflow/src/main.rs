fn main() {
    // let x=4;

    // if x!=0 {
    //     println!("Number is other than 0");
    // }
    // else if x<4 {
    //     println!("Number is less than 4");
    // }
    // else {
    //     println!("Number is 4");
    // }

    // if else is an expression by which if else can return the values we can write if else like

    // let y: i32=40;

    // let x: bool = if y==40 {true} else {false}; 

    // -------------------- Loops ------------------//

    // 1. loop this is and indefinite loop (infinite) we can stop it with the help of break condition && loop is also an expression which means it can return the value

    // loop {
    //     println!("this is a loop");
    // }

    // let mut num=1;

    // let res=loop {
    //     if num>10 {
    //         break 40;
    //     }
    //     println!("num is {num}");
    //     num=num+num;

    // };

    // println!("Res value is {res}");

    // we can label the loops if we have more than one loop

    // let mut num=1;
    // 'myloop: loop {  // this is the label
    //     if num>10 {
    //         break;
    //     }
    //     println!("num is {num}");
    //     num=num+1;
        
    //     loop {
    //         if num>100 {
    //             // break;  // this will only break the inner loop but if we specify the label of the loop which we have to break it can break that loop
    //             break 'myloop;  // this will break the loop with the specified label
    //         }
    //     }
    // }

    // while loop atleast if you are a programmer then you know this mf! i dont have to describe about this
    // let mut num=0;

    // while num<10 {
    //     println!("I am a programmer");
    //     num=num+1;
    // }

    // for loop 

    // let arr=[1,2,3,4,5];

    // for element in arr {
    //     println!("{element}");
    // }

    // we can also make range in for loop

    for element in (1..=10).rev() {
        println!("{element}");
    }

}
