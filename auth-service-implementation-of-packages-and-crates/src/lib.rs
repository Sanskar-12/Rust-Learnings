// by default all the library are private to lib.rs file but u can make it public by using pub

// now we can also group all the functions with mod
#![allow(dead_code,unused_variables)]

mod database;

pub mod auth_utils;

use database::{connect_to_database,DbStatus};
use auth_utils::login;

pub fn authenticate_user(credentials:auth_utils::models::Credentials) {
    if let DbStatus::Connected = connect_to_database() {
        login(credentials)
    }
}

// mod util; // using this we can scatter the library crates function all over the files
// this will include the util as a library crate inside the tree




