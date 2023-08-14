extern crate dotenv;

use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database = env::var("DATABASE_URL").expect("DATABASE environment variable");
    MysqlConnection::establish(&database)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database))
    //let connection = MysqlConnection::establish(&database).expect("Failed to connect to database");
}
