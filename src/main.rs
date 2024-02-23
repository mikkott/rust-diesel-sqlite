use self::db::models::{NewUser, User};
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;
use std::env;

pub mod db;

fn main() {
    dotenv().ok();

    let database_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn: &mut SqliteConnection = &mut SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    let user: User = User::new("foo@bar", "foobarri");

    let user: Option<User> = match exist_user(conn, user.email.as_str()) {
        Some(false) => Some(create_user(conn, &user.username, &user.email)),
        Some(true) => None,
        None => None,
    };

    match user {
        Some(user) => println!("inserted {}", user.email),
        None => println!("user exists, no action"),
    };
}

pub fn exist_user(conn: &mut SqliteConnection, e: &str) -> Option<bool> {
    use self::db::schema::users::dsl::*;
    use diesel::dsl::exists;

    let user_exists: Result<bool, diesel::result::Error> = diesel::select(exists(
        users.filter(self::db::schema::users::dsl::email.eq(&e)),
    ))
    .get_result::<bool>(conn);

    match user_exists {
        Ok(result) => Some(result),
        Err(_) => panic!(),
    }
}

pub fn create_user(conn: &mut SqliteConnection, username: &str, email: &str) -> User {
    use crate::db::schema::users;

    let new_user = NewUser { username, email };

    diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(conn)
        .expect("err")
}

impl User {
    fn new(email: &str, username: &str) -> Self {
        Self {
            email: email.to_string(),
            username: username.to_string(),
            id: 0,
            enabled: false,
        }
    }
}
