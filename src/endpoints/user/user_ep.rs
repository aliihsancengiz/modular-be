use crate::managers::um::UserManager;
use crate::models::user::{UserLogin, UserRegister};

pub struct UserEndpoint {
    um: UserManager,
}

impl UserEndpoint {
    pub fn new() -> Self {
        Self {
            um: UserManager::new(),
        }
    }

    pub fn login(&mut self, user: UserLogin) -> Option<String> {
        match self.um.find_by_username(&user.username) {
            Some(x) => {
                if x.password == user.password {
                    Some("Successfully logged in.".to_string())
                } else {
                    None
                }
            }
            None => None,
        }
    }

    pub fn register(&mut self, user: UserRegister) -> Option<String> {
        match self.um.find_by_username(&user.username) {
            Some(_) => None,
            None => match self.um.insert(user) {
                Ok(_) => Some("Successfully registered user.".to_string()),
                Err(_) => None,
            },
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
