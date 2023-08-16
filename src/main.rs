#![allow(
    dead_code,
    unused_crate_dependencies,
    unused_variables,
    unused_assignments,
    unused,
    non_snake_case,
    non_camel_case_types,
    warnings
)]

#[macro_use]
extern crate rocket;
use rocket::{Build, Rocket};

use diesel::prelude::*;
use diesel::MysqlConnection;
use dotenv::dotenv;
use std::env;

mod middleware;
mod controller;
mod database;
mod models;
mod schema;

// fn main() {
//     println!("Hello, world!");
// }

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount(
            "/",
            routes![
                controller::get_post,
                controller::create_post,
                controller::delete_post
            ],
        )
        .mount("/api/", routes![controller::validate_post])
        .launch()
        .await
        .expect("Failed to launch Rocket");
    Ok(())
}
