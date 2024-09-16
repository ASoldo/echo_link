use crate::config::db::DbPool;
use crate::models::User;
use crate::schema::users;
use actix::{Actor, StreamHandler};
use actix_web::HttpRequest;
use actix_web::{web, Error, HttpResponse};
use actix_web_actors::ws;
use bcrypt::{hash, DEFAULT_COST};
use chrono::Utc;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInput {
    pub username: String,
    pub email: String,
    pub password: String,
}

// **Add `pub` to make the function public**
pub async fn create_user(
    pool: web::Data<DbPool>,
    item: web::Json<UserInput>,
) -> Result<HttpResponse, Error> {
    let mut conn = pool.get().expect("Couldn't get DB connection from pool");

    // Hash the password
    let hashed_password = hash(&item.password, DEFAULT_COST).unwrap();

    // Create new user instance
    let new_user = User {
        id: Uuid::new_v4(),
        username: item.username.clone(),
        email: item.email.clone(),
        password_hash: hashed_password,
        created_at: Utc::now().naive_utc(),
    };

    // Insert into the database
    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&mut conn)
        .expect("Error inserting new user");

    Ok(HttpResponse::Ok().json(&new_user))
}

// **Add `pub` to make the function public**
pub async fn get_users(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    use crate::schema::users::dsl::*;

    let mut conn = pool.get().expect("Couldn't get DB connection from pool");

    let all_users = users
        .load::<User>(&mut conn) // Pass mutable reference here
        .expect("Error loading users");

    Ok(HttpResponse::Ok().json(all_users))
}

// **Add `pub` to make the function public**
pub async fn ws_index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(MyWebSocket::new(), &req, stream)
}

// WebSocket handler
struct MyWebSocket;

impl MyWebSocket {
    fn new() -> Self {
        Self
    }
}

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}
