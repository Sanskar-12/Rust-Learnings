// by default all the library are private to lib.rs file but u can make it public by using pub

// now we can also group all the functions with mod
#![allow(dead_code,unused_variables)]

mod database {
    pub enum DbStatus {
        Connected,
        Interrupted
    }
    
    pub fn connect_to_database() ->DbStatus {
        // connect to db....
        DbStatus::Connected
    }
    
    pub fn get_data() {
        // fetch the user data from the database
    }
}

pub mod auth_utils {
    pub fn login(cred:models::Credentials) {
        // login the user
        crate::database::get_data()  // crate means you have start from the root
    }

    pub mod models { // we can make nested modules
        pub struct Credentials {
            pub email:String,
            pub password:String
        }
    }
}

pub fn authenticate_user(credentials:auth_utils::models::Credentials) {
    if let crate::database::DbStatus::Connected = crate::database::connect_to_database() {
        auth_utils::login(credentials)
    }
}





