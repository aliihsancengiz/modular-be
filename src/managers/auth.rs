use crate::managers::token::Tokeniser;
use crate::models::user::UserRole;
use actix_web::error::ErrorUnauthorized;
use actix_web::{dev, Error, FromRequest, HttpRequest};
use futures::future::{err, ok, Ready};

pub struct AsAdmin;
pub struct AsUser;
pub struct AsViewer;

impl FromRequest for AsAdmin {
    type Error = Error;
    type Future = Ready<Result<AsAdmin, Error>>;

    fn from_request(_req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        let _auth = _req.headers().get("Authorization");
        match _auth {
            Some(_) => {
                let _split: Vec<&str> = _auth.unwrap().to_str().unwrap().split("Bearer").collect();
                let token = _split[1].trim();
                let tokeniser = Tokeniser::new();
                match tokeniser.validate_token(token.to_string()) {
                    Some(r) => match r {
                        UserRole::ADMIN => ok(AsAdmin),
                        _ => err(ErrorUnauthorized("Unauthorized Role")),
                    },
                    None => err(ErrorUnauthorized("Invalid Token!")),
                }
            }
            None => err(ErrorUnauthorized("Invalid Token!")),
        }
    }
}

impl FromRequest for AsUser {
    type Error = Error;
    type Future = Ready<Result<AsUser, Error>>;

    fn from_request(_req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        let _auth = _req.headers().get("Authorization");
        match _auth {
            Some(_) => {
                let _split: Vec<&str> = _auth.unwrap().to_str().unwrap().split("Bearer").collect();
                let token = _split[1].trim();
                let tokeniser = Tokeniser::new();
                match tokeniser.validate_token(token.to_string()) {
                    Some(r) => match r {
                        UserRole::ADMIN => ok(AsUser),
                        UserRole::USER => ok(AsUser),
                        _ => err(ErrorUnauthorized("Unauthorized Role")),
                    },
                    None => err(ErrorUnauthorized("Invalid Token!")),
                }
            }
            None => err(ErrorUnauthorized("Invalid Token!")),
        }
    }
}

impl FromRequest for AsViewer {
    type Error = Error;
    type Future = Ready<Result<AsViewer, Error>>;

    fn from_request(_req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        let _auth = _req.headers().get("Authorization");
        match _auth {
            Some(_) => {
                let _split: Vec<&str> = _auth.unwrap().to_str().unwrap().split("Bearer").collect();
                let token = _split[1].trim();
                let tokeniser = Tokeniser::new();
                match tokeniser.validate_token(token.to_string()) {
                    Some(r) => match r {
                        UserRole::ADMIN => ok(AsViewer),
                        UserRole::USER => ok(AsViewer),
                        UserRole::VIEWER => ok(AsViewer),
                        _ => err(ErrorUnauthorized("Unauthorized Role")),
                    },
                    None => err(ErrorUnauthorized("Invalid Token!")),
                }
            }
            None => err(ErrorUnauthorized("Invalid Token!")),
        }
    }
}
