use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserRegister {
	#[serde(skip_serializing)]
	#[serde(skip_deserializing)]
    pub user_id: String,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize,Deserialize)]
pub struct UserLogin {
	pub username : String,
	pub password : String,
}