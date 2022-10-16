use diesel::prelude::*;

pub struct DatabaseManager;

impl DatabaseManager{	
	pub fn establish_connection(db_url : String) -> SqliteConnection {
		
		// let database_url = "/home/ali/Workspace/examples/rust-ex/userDb/database.db";
		let database_url = db_url.as_str();
		
		SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
	}
	
}