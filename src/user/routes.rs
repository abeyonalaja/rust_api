use crate::user::User;
use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use serde_json::json;

#[get("/users")]
