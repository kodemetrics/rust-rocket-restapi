use crate::schema::*;
use diesel::prelude::*;
// use rocket::form::validate;
use serde::*;
use validator::*;
// use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Validate, Queryable, Insertable, Selectable)]
#[table_name = "posts"]
pub struct Post {
    pub id: i32,
    #[validate(length(min = 5))]
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct Person {
    pub name: String,
    pub age: u8,
}

// #[derive(Deserialize, Insertable)]
// #[table_name = "posts"]
// pub struct NewPost {
//     pub id: i32,
//     pub title: String,
//     pub body: String,
//     pub published: bool,
// }

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct User {
    pub full_name: String,
    pub username: String,
    pub email: String,
    pub password: String,
}
