pub mod auth;
pub mod model;

use rocket::{self, http::Status};
use rocket_contrib::json::{Json, JsonValue};
use crate::db::PostgresDatabaseConnection;
use model::User;
use auth::create_token;
use auth::Username;

#[derive(Serialize, Deserialize)]
struct Credentials {
   username: String,
   password: String
}

#[get("/")]
fn list(connection: PostgresDatabaseConnection) -> Json<Vec<User>> {
    Json(User::list(&connection))
}

#[get("/<id>")]
fn get(id: i32, connection: PostgresDatabaseConnection) -> Json<User> {
    println!("Trying to get user with id {}", id);
    // Json(User::new(0, "test", "1234567890"))
    Json(User::get(id, &connection).unwrap())
}

#[get("/")]
fn read(username: Username, connection: PostgresDatabaseConnection) -> Result<Json<User>, Status> {
    User::by_name(&username.0[..], &connection)
        .map(|user|Json(user))
        .ok_or_else(||Status::NotFound)
}

#[get("/", rank = 2)]
fn read_error() -> Json<JsonValue> {
    Json(json!(
        {
            "success": false,
            "message": "Not authorized"
        }
    ))
}

#[post("/", data = "<credentials>")]
fn login(credentials: Json<Credentials>, connection: PostgresDatabaseConnection) -> Result<Json<String>, Status> {
    match User::by_name_and_password(&credentials.username[..], &credentials.password[..], &connection) {
        Some(user) => {
            let token = create_token(&user.name[..]);
            Ok(Json(token))
        },
        None => {
            Err(Status::new(400, "Invalid login or password"))
        }
    }
}

pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket
        .mount("/user", routes![read, read_error, get])
        .mount("/users", routes![list])
        .mount("/auth", routes![login])
}
