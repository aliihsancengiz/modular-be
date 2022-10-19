use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct UserLoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserLoginResponse {
    pub token: String,
}
