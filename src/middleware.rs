use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Request, Response};
use rocket::http::Header;
use rocket::http::Status;

pub struct CustomMiddleware;
