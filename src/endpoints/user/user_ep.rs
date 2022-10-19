use crate::managers::um::UserManager;
use crate::models::user::{User, UserRegister};
use crypto::digest::Digest;
use crypto::sha2::Sha256;

pub struct UserEndpoint {
    pub um: UserManager,
}

impl UserEndpoint {
    pub fn new() -> Self {
        Self {
            um: UserManager::new(),
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

    pub fn all(&mut self) -> Option<Vec<User>> {
        match self.um.all() {
            Ok(res) => Some(res),
            Err(_) => None,
        }
    }
}
