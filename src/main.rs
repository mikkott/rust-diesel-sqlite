use self::db::models::User;
use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;

pub mod db;

fn main() {
    dotenv().ok();

    let conn: &mut SqliteConnection = &mut db::conn::conn_sqlite();

    let new_user: User = db::models::User::new("delete@this", "testuser");

    let user_to_delete = &new_user;

    let user: Option<User> = match db::controllers::exist_user(conn, new_user.email.as_str()) {
        Some(false) => Some(db::controllers::create_user(
            conn,
            &new_user.username,
            &new_user.email,
        )),
        Some(true) => None,
        None => None,
    };

    match user {
        Some(user) => println!("inserted {}", user.email),
        None => println!("user exists, no action"),
    };

    let foo: Result<Vec<String>, diesel::result::Error> = db::controllers::list_users(conn);

    match foo {
        Ok(res) => {
            for e in res {
                println!("{}", e);
            }
        }
        Err(_) => println!("Error"),
    };

    let res = db::controllers::delete_user(
        conn,
        User {
            id: 1,
            username: "foo".to_string(),
            email: user_to_delete.email.clone(),
            enabled: false,
        },
    );

    let foo: Result<Vec<String>, diesel::result::Error> = db::controllers::list_users(conn);

    match foo {
        Ok(res) => {
            if res.len() > 0 {
                for e in res {
                    println!("{}", e);
                }
            } else {
                println!("db empty");
            }
        }
        Err(_) => println!("Error"),
    };

    match res {
        Ok(_) => println!("delete test success"),
        Err(_) => println!("delete test bad"),
    }
}
