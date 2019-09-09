extern crate argon2;
extern crate jsonwebtoken as jwt;

use argon2::{Config};
use blake2::{Blake2b, Digest};
use jwt::{encode, decode, Header, Algorithm, Validation};
use serde_derive::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};
use std::ops::Add;

use rocket::Outcome;
use rocket::request::{self, Request, FromRequest};

pub struct Username(pub String);

impl<'a, 'r> FromRequest<'a, 'r> for Username {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Username, ()> {
        let keys: Vec<_> = request.headers().get("Authentication").collect();
        if keys.len() != 1 {
            return Outcome::Forward(());
        }
        match read_username_from_token(keys[0]) {
            Ok(claim) => Outcome::Success(Username(claim)),
            Err(_) => Outcome::Forward(())
        }
    }
}

pub fn read_username_from_token(token: &str) -> Result<String, String> {
    match decode::<Claims>(&token, "secret".as_ref(), &Validation::default()) {
        Ok(token_data) => {
            Ok(token_data.claims.sub)
        },
        Err(_) => {
            Err("Wrong token".to_owned())
        }
    }
}

pub fn create_token(username: &str) -> String {
    let mut header = Header::default();
    header.kid = Some("sometext".to_owned());
    header.alg = Algorithm::HS256;
    let expiration = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().add(Duration::from_secs(3600));
    encode(&header, &Claims{
        iss: "TestApp".to_owned(),
        sub: username.to_owned(),
        exp: expiration.as_secs() as usize
    }, "secret".as_ref()
    ).expect("Filed to generate new token")
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    iss: String,
    sub: String,
    exp: usize,
}

pub fn encode_password(user: &str, password: &str) -> String {
    let config = Config::default();
    let salt = Blake2b::digest(user.as_bytes());
    argon2::hash_encoded(password.as_bytes(), &salt[..], &config).unwrap()
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_encode_password() {
        let hash = encode_password("User01", "012345");
        assert_eq!(hash, "$argon2i$v=19$m=4096,t=3,p=1$L/zEMVPXiFuwRKmjBwAF4w1GhZV/qkiE6JCM8CDTMDcTOJZ1gg6SzYxNZUTbS8Je3e+c5nfZebhq7UXTkemKzw$vXHingF6NqF18FnaZXcldHVT0ukNvjVqHEePoKCWV0U");
    }

}
