use crate::managers::token::Claims;
use actix_web::error::ErrorUnauthorized;
use actix_web::{dev, Error, FromRequest, HttpRequest};
use envfile::EnvFile;
use futures::future::{err, ok, Ready};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use std::path::Path;

pub struct AuthorizationService;

impl FromRequest for AuthorizationService {
    type Error = Error;
    type Future = Ready<Result<AuthorizationService, Error>>;

    fn from_request(_req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        let _auth = _req.headers().get("Authorization");
        match _auth {
            Some(_) => {
                let _split: Vec<&str> = _auth.unwrap().to_str().unwrap().split("Bearer").collect();
                let token = _split[1].trim();
                let env = EnvFile::new(&Path::new("config.env")).unwrap();
                let secret = env.get("SECRET").unwrap();
                match decode::<Claims>(
                    token,
                    &DecodingKey::from_secret(secret.as_bytes()),
                    &Validation::new(Algorithm::HS256),
                ) {
                    Ok(_token) => ok(AuthorizationService),
                    Err(_e) => err(ErrorUnauthorized("Invalid Token!")),
                }
            }
            None => err(ErrorUnauthorized("Invalid Token!")),
        }
    }
}
