use crate::managers::token::Tokeniser;
use actix_web::error::ErrorUnauthorized;
use actix_web::{dev, Error, FromRequest, HttpRequest};
use futures::future::{err, ok, Ready};

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
                let tokeniser = Tokeniser::new();
                match tokeniser.validate_token(token.to_string()) {
                    true => ok(AuthorizationService),
                    false => err(ErrorUnauthorized("Invalid Token!")),
                }
            }
            None => err(ErrorUnauthorized("Invalid Token!")),
        }
    }
}
