use diesel::prelude::*;
use diesel::Identifiable;
use rocket::http::Status;
use rocket::response::status;
use rocket::response::status::NoContent;
use rocket::serde::json::Json;
use rocket::Route;
use validator::*;

use crate::database;
use crate::models::*;
use crate::schema::posts;

#[get("/")]
pub fn get_post() -> Result<Json<Vec<Post>>, status::Custom<String>> {
    use crate::schema::posts::dsl::posts;
    let connection = &mut database::establish_connection();
    match posts.load::<Post>(connection) {
        Ok(posts_list) => Ok(Json(posts_list)),
        Err(_) => Err(status::Custom(
            Status::InternalServerError,
            "Error loading posts".into(),
        )),
    }
}

#[delete("/<post_id>")]
pub fn delete_post(post_id: i32) -> Status {
    use crate::schema::posts::dsl::*;
    let connection = &mut database::establish_connection();
    diesel::delete(posts.filter(id.eq(post_id)))
        .execute(connection)
        .expect("Error deleting Post");

    Status::Ok
}

#[get("/<post_id>")]
pub fn get_post_by_id(post_id: i32) -> Json<Post> {
    use crate::schema::posts::dsl::*;
    let connection = &mut database::establish_connection();

    let result = posts
        .filter(id.eq(post_id))
        .load::<Post>(connection)
        .expect("Error geting Post");

       let post = posts
        .find(post_id)
        .select(Post::as_select())
        .first(connection)
        .optional();
    //  Json(post)
    //  Json(result)
}

#[post("/", data = "<post>")]
pub fn update_post(post: Json<Post>) -> Json<Post> {
    let connection = &mut database::establish_connection();

    Json(
        posts::table
            .order(posts::id.desc())
            .first(connection)
            .unwrap(),
    )
}

#[post("/", data = "<post>")]
pub fn create_post(post: Json<Post>) -> Json<Post> {
    let connection = &mut database::establish_connection();

    let new_post = Post {
        id: 0, // Auto-incremented by the database
        title: "New Post Title".to_string(),
        body: "This is the body of the new post.".to_string(),
        published: false,
    };

    diesel::insert_into(posts::table)
        .values(post.into_inner())
        .execute(connection)
        .expect("Error Adding Post");

    Json(
        posts::table
            .order(posts::id.desc())
            .first(connection)
            .unwrap(),
    )
}

// #[post("/", format = "json", data = "<person>")]
// pub fn validate_post(person: Json<Person>) -> Json<Person> {
//     let post_data = person.into_inner();
//     match post_data.validate() {
//         Ok(_) => {},
//         Err(_) => (),
//     }
// }

#[post("/")]
pub fn validate_post() -> &'static str {
    "Hello, world!"
}

// #[post("/", format = "json",data = "<post>")]
// pub fn validate_post(post: Json<Post>) -> Result<Status, Json<Vec<String>>> {
//     let post_data = post.into_inner();
//     match post_data.validate() {
//         Ok(_) => {
//             println!("Validated Post: {:?}", 1);
//             Ok(Status::Created)
//         }
//         Err(validation_errors) => {
//               let error_messages: Vec<String> = validation_errors
//                 .field_errors()
//                 .values()
//                 .flatten()
//                 .map(|error| error.message.to_string())
//                 .collect();
//              Err(Json(error_messages))
//         }
//     }
// }

pub fn index() -> Json<Vec<Post>> {
    use crate::schema::posts::dsl::*;
    let connection = &mut database::establish_connection();
    posts
        .load::<Post>(connection)
        .map(Json)
        .expect("Error loading Post")
}


