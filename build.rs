use std::process::Command;
use envfile::EnvFile;
use std::path::Path;

fn main(){
	let env = EnvFile::new(&Path::new("config.env")).unwrap();
	let database_url = env.get("DATABASE_PATH").unwrap().to_string();

	Command::new("diesel").args(["migration", "run", "--database-url", database_url.as_str()]).output().unwrap();
}

