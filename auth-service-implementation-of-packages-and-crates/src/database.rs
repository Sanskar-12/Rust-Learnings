

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