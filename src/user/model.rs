use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use crate::user::auth::{encode_password};
use crate::schema::users;

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    #[serde(skip_serializing)]
    pub password_hash: String
}

impl User {

    pub fn list(connection: &PgConnection) -> Vec<User> {
        let result = users::table.load::<User>(connection).expect("Error loading users");
        result
    }

    pub fn new(id: i32, name: &str, password_hash: &str) -> User {
        User{id: id, name: name.to_owned(), password_hash: password_hash.to_owned()}
    }

    pub fn by_name_and_password(name: &str, password: &str, connection: &PgConnection) -> Option<User> {
        users::table
            .filter(users::name.eq(name))
            .filter(users::password_hash.eq(encode_password(name, password)))
            .first::<User>(connection).optional().expect("User not found")
    }

    pub fn by_name(name: &str, connection: &PgConnection) -> Option<User> {
        users::table
            .filter(users::name.eq(name))
            .first::<User>(connection).optional().expect("User not found")
    }

    pub fn get(id: i32, connection: &PgConnection) -> Option<User> {
        users::table.find(id).first(connection).optional().expect("User not found")
    }

}
