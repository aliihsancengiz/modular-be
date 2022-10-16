use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use crate::schema::schema::users;

#[derive(Queryable,Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Insertable,Serialize,Deserialize)]
#[diesel(table_name = users)]
pub struct UserRegister
{
	pub username : String,
	pub email : String,
	pub password : String,
}

#[derive(Serialize,Deserialize)]
pub struct UserLogin {
	pub username : String,
	pub password : String,
}