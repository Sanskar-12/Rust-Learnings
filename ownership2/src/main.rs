// fn gives_ownership() -> String{
//     let str=String::from("gives ownership"); // scope of str is limited to this function only
//     return str;
// }

// fn takes_ownership(str:String) {
//     println!("{str}");
// }

// fn takes_ownership_and_gives_back(str:String) -> String {
//     str
// }

fn calculate_len(s:String) -> (String,usize) {
    let len=s.len();
    (s,len)
}

fn main() {
    // let s1=gives_ownership(); // this function gives ownership to s1
    // // println!("{s1}");

    // let s2=String::from("takes ownership"); 
    // takes_ownership(s2); // this will take the ownership of s2 to str
    
    // // println!("{s2}");  this will give error because s2's ownership has been moved to str inside the function takes_ownership
    
    // let s3=String::from("takes ownership and gives back ownership"); 
    // let s4=takes_ownership_and_gives_back(s3); // this will take the ownership of s3 and gives it back to s4
    // println!("{s4}"); // if we put s3 then this will give error because s3 ownership has been moved to str inside th function takes_ownership_and_gives_back

    // --------------------------------------------------------------//

    let str=String::from("name");
    let (str,len)=calculate_len(str);
    // println!("The length of {str} is {len}"); // this will give error because the ownership of str is being moved
    println!("The length of {str} is {len}"); // this will not give error because we have returned the  ownership of str 
}


