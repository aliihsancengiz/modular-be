use crate::schema::schema::users;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Insertable, Clone, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct User {
    #[serde(skip)]
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
}
