mod model;

use rocket_contrib::json::Json;
use rocket::{self, http::Status};
use serde_derive::{Deserialize};
use crate::db::PostgresDatabaseConnection;
use crate::user::auth::Username;
use crate::user::model::User;

use self::model::Post;
use self::model::NewPost;
use self::model::UpdatePost;

#[derive(Deserialize)]
struct NewPostRequest {
    title: String,
    body: String
}

#[derive(Deserialize)]
struct UpdatePostRequest {
    id: i32,
    title: Option<String>,
    body: Option<String>
}

#[get("/")]
fn list(username: Username, connection: PostgresDatabaseConnection) -> Json<Vec<Post>> {
    let user_id = User::by_name(&username.0[..], &connection).unwrap().id;
    Json(Post::find_by_user_id(user_id, &connection))
}

#[get("/<id>")]
fn read(id: i32, _username: Username, connection: PostgresDatabaseConnection) -> Result<Json<Post>, Status> {
    Post::find_by_id(id, &connection)
        .map(|post|Json(post))
        .ok_or_else(||Status::NotFound)
}

#[post("/", data = "<post>")]
fn create(post: Json<NewPostRequest>, username: Username, connection: PostgresDatabaseConnection) -> Result<Status, Status> {
    let user_id = User::by_name(&username.0[..], &connection).unwrap().id;
    Post::insert(&NewPost{title: &post.title[..], body: &post.body[..], user_id: user_id}, &connection)
        .map(|_|Status::Created)
        .map_err(|_|Status::InternalServerError)
}

#[put("/", data = "<post>")]
fn update(post: Json<UpdatePostRequest>, username: Username, connection: PostgresDatabaseConnection) -> Result<Status, Status> {
    let user_id = User::by_name(&username.0[..], &connection).unwrap().id;
    Post::update(&UpdatePost{title: post.title.as_ref().map(|t|&t[..]), body: post.body.as_ref().map(|t|&t[..])}, post.id, user_id, &connection)
        .map(|_|Status::Created)
        .map_err(|_|Status::InternalServerError)
}

pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket
        .mount("/post", routes![read, create, update])
        .mount("/posts", routes![list])
}
