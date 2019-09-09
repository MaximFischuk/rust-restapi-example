#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;

mod db;
mod user;
mod post;
mod schema;

use std::{thread, time};
use db::*;

#[get("/")]
fn ok() -> &'static str {
    "Ok"
}

#[get("/sleep/<seconds>")]
fn sleep(seconds: u64) {
    thread::sleep(time::Duration::from_secs(seconds));
}

fn main() {
    let mut r = rocket::ignite()
        // .manage(db::establish_connection())
        .attach(PostgresDatabaseConnection::fairing())
        .mount("/", routes![ok, sleep]);

    r = user::mount(r);
    r = post::mount(r);

    r.launch();
}
