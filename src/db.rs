extern crate dotenv;

use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use rocket_contrib::databases::diesel;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};
use rocket::http::Status;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

pub type PsqlPool = Pool<ConnectionManager<PgConnection>>;

#[database("postgres")]
pub struct PostgresDatabaseConnection(diesel::PgConnection);

pub struct Connection(pub PooledConnection<ConnectionManager<PgConnection>>);

pub fn establish_connection() -> PsqlPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    Pool::new(manager).expect(&format!("Error connecting to {}", database_url))
}

impl<'a, 'r> FromRequest<'a, 'r> for Connection {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let pool = request.guard::<State<PsqlPool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(Connection(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}
