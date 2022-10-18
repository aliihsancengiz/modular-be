use crate::models::user::User;
use chrono::{Duration, Utc};
use envfile::EnvFile;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub email: String,
}

pub struct Tokeniser {
    pub secret: String,
}

impl Tokeniser {
    pub fn new() -> Self {
        let env = EnvFile::new(&Path::new("config.env")).unwrap();
        let secret = env.get("SECRET").unwrap().to_string();
        Self { secret }
    }

    pub fn generate_tokens(&self, user: User, expiration_in_hours: u16) -> String {
        let _date = Utc::now() + Duration::hours(expiration_in_hours as i64);

        let my_claims = Claims {
            sub: user.username,
            exp: _date.timestamp() as usize,
            email: user.email,
        };
        encode(
            &Header::default(),
            &my_claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .unwrap()
    }

    pub fn validate_token(&self, token: String) -> bool {
        let _decode = decode::<Claims>(
            token.as_str(),
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::new(Algorithm::HS256),
        );
        match _decode {
            Ok(decoded) => {
                println!("{}", decoded.claims.sub.to_string());
                true
            }
            Err(_) => {
                println!("Error");
                false
            }
        }
    }
}
