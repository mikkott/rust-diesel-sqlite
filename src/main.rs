use self::db::models::User;
use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;

pub mod db;

fn main() {
    dotenv().ok();

    let conn: &mut SqliteConnection = &mut db::conn::conn_sqlite();

    let user: User = db::models::User::new("foo@bar", "foobarri");

    let user: Option<User> = match db::controllers::exist_user(conn, user.email.as_str()) {
        Some(false) => Some(db::controllers::create_user(
            conn,
            &user.username,
            &user.email,
        )),
        Some(true) => None,
        None => None,
    };

    match user {
        Some(user) => println!("inserted {}", user.email),
        None => println!("user exists, no action"),
    };
}
