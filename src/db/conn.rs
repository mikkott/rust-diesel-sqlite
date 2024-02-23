use diesel::{sqlite::SqliteConnection, Connection};
use std::env;
pub fn conn_sqlite() -> SqliteConnection {
    let database_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    diesel::sqlite::SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
