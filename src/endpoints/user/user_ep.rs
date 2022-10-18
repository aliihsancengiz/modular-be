use crate::managers::token::Tokeniser;
use crate::managers::um::UserManager;
use crate::models::user::{UserLogin, UserLoginResponse, UserRegister};
use crypto::digest::Digest;
use crypto::sha2::Sha256;

pub struct UserEndpoint {
    um: UserManager,
    tokeniser: Tokeniser,
}

impl UserEndpoint {
    pub fn new() -> Self {
        Self {
            um: UserManager::new(),
            tokeniser: Tokeniser::new(),
        }
    }

    pub fn login(&mut self, user: UserLogin) -> Option<UserLoginResponse> {
        match self.um.find_by_username(&user.username) {
            Some(_usr) => {
                let mut sha = Sha256::new();
                sha.input_str(user.password.as_str());

                if _usr.password == sha.result_str() {
                    Some(UserLoginResponse {
                        token: self.tokeniser.generate_tokens(_usr, 2),
                    })
                } else {
                    None
                }
            }
            None => None,
        }
    }

    pub fn register(&mut self, user: &mut UserRegister) -> Option<String> {
        match self.um.find_by_username(&user.username) {
            Some(_) => None,
            None => {
                let mut sha = Sha256::new();
                sha.input_str(user.password.as_str());
                user.password = sha.result_str();
                match self.um.insert(user) {
                    Ok(_) => Some("Successfully registered user.".to_string()),
                    Err(_) => None,
                }
            }
        }
    }

    pub fn all(&mut self) {
        let results = self.um.all().unwrap();
        println!("Displaying {} User", results.len());
        for user in results {
            println!(
                "Id : {}, Name {} , Email {} , Password {}",
                user.id, user.username, user.email, user.password
            );
        }
    }
}
