use envfile::EnvFile;
use std::path::Path;
use std::result::Result;

use diesel::insert_into;
use diesel::prelude::*;

use crate::managers::db::DatabaseManager;
use crate::models::user::{User, UserRegister};
use crate::schema::schema::users::dsl::users;
use crate::schema::schema::users::*;

pub struct UserManager {
    connection: SqliteConnection,
}

impl UserManager {
    pub fn new() -> Self {
        let env = EnvFile::new(&Path::new("config.env")).unwrap();
        let database_url = env.get("DATABASE_PATH").unwrap().to_string();
        Self {
            connection: DatabaseManager::establish_connection(database_url),
        }
    }

    pub fn insert(&mut self, user: &UserRegister) -> Result<usize, diesel::result::Error> {
        insert_into(users)
            .values(user)
            .execute(&mut self.connection)
    }

    pub fn all(&mut self) -> QueryResult<Vec<User>> {
        users.load::<User>(&mut self.connection)
    }

    pub fn update(&mut self, usr: User) {
        diesel::update(users.filter(id.eq(usr.id)))
            .set((
                username.eq(usr.username),
                email.eq(usr.email),
                password.eq(usr.password),
            ))
            .execute(&mut self.connection)
            .unwrap();
    }

    pub fn find_by_username(&mut self, user_name: &String) -> Option<User> {
        match users
            .filter(username.eq(user_name))
            .first(&mut self.connection)
        {
            Ok(x) => Some(x),
            Err(_) => None,
        }
    }
}
