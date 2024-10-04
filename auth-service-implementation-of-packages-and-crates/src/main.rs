// we can make use keyword to import the libraries crate which is made public 

// if both the library comes from same package then we can destructure it like

// use auth_service_implementation_of_packages_and_crates::{
//     Credentials,
//     authenticate_user
// }; // like this

use auth_service_implementation_of_packages_and_crates::authenticate_user;
use auth_service_implementation_of_packages_and_crates::auth_utils::models::Credentials;

fn main() {
    let cred = Credentials {
        email:String::from("sanskar@gmail.com"),
        password:String::from("password"),
    };

    authenticate_user(cred);
}
