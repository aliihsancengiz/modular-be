use crate::schema::schema::users;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Insertable, Clone, Serialize, Deserialize, Debug)]
#[diesel(table_name = users)]
pub struct User {
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: String,
}

impl User {
    pub fn role_from_str(&self) -> UserRole {
        match self.role.clone().as_str() {
            "ADMIN" => UserRole::ADMIN,
            "USER" => UserRole::USER,
            "VIEWER" => UserRole::VIEWER,
            &_ => UserRole::NONE,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum UserRole {
    ADMIN,
    USER,
    VIEWER,
    NONE,
}
