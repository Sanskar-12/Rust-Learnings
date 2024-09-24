// structs are similar to instance in typescript in which we can group multiple things in one object

// defining the struct
struct User {
    username:String,
    email:String,
    phone:u64
}

// fn main() {
//     // create an instance of User Struct
//     let mut user: User= User {
//         username:String::from("Sanskar12"),
//         email:String::from("sanskar@gmail.com"),
//         phone:9321597048
//     };

//     // reading the value of user
//     println!("Username of the user is {}",user.username);
    
//     // mutate the value the username
//     user.username=String::from("Sanskar");
//     println!("Username of the user is {}",user.username);

//     let new_owner = user.username;
//     user.username=String::from("New owner"); // in this i am assigning the new value as owner of user.username thats why its not giving errors

//     // println!("Username of the user is {}",user.username); // now we cannot access this because ownership is moved 
// }

// creating the another instance with the use of other instance with update syntax
fn main() {
    let user: User = build_user(
        String::from("Sanskar"),
        String::from("sanskar@gmail.com"),
        9792937239,
    );

    // i can update like this but we have more properties then we can use shorthand like spread operator like in javascript
    // let user2=User {
    //     username:user.username,
    //     email:String::from("sanskar123@gmail.com"),
    //     phone:user.phone
    // };

    let user2=User {
        email:String::from("sanskar123@gmail.com"),
        ..user // spread operator
    };

    // println!("Username is {}",user.username); // i cant use it because the ownership is with user2 now
}


fn build_user(username:String, email:String, phone:u64) -> User {
    User {
        username,
        email,
        phone
    }
}
