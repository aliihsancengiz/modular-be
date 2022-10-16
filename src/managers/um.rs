#[warn(dead_code)]

use diesel::prelude::*;
use diesel::insert_into;

use crate::schema::schema::users::dsl::users;
use crate::models::user::{User,UserRegister};

use crate::managers::db::DatabaseManager;

use crate::schema::schema::users::*;

use std::result::Result;

pub struct UserManager
{
	connection:SqliteConnection,
}

impl UserManager
{
	pub fn new() -> Self
	{
		// Todo : move this to env file
		let database_url = "/home/ali/Workspace/examples/rust-ex/userDb/database.db".to_string();
		Self {
			connection : DatabaseManager::establish_connection(database_url),
		}
	}

	pub fn insert(&mut self,user:UserRegister)
	{
		insert_into(users).values(&user).execute(&mut self.connection).unwrap();
	}

	pub fn all(&mut self) -> QueryResult<Vec<User>>
	{
		users.load::<User>(&mut self.connection)
	}

	pub fn update(&mut self,usr:User)
	{
		diesel::update(users.filter(id.eq(usr.id)))
		.set((username.eq(usr.username),email.eq(usr.email),password.eq(usr.password)))
		.execute(&mut self.connection).unwrap();
	}

	pub fn find(&mut self,_id : i32) -> Result<User,bool>
	{
		if let Ok(res) = users.find(_id).load::<User>(&mut self.connection){
			let usr = res.first().cloned();
			Ok(usr.unwrap())
		}
		else {
			Err(false)
		}
	}
}