use crate::db::models::NewUser;
use crate::db::models::User;
use crate::db::schema::users::email;

use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::SqliteConnection;

pub fn exist_user(conn: &mut SqliteConnection, e: &str) -> Option<bool> {
    use crate::db::schema::users::dsl::*;
    use diesel::dsl::exists;

    let user_exists: Result<bool, diesel::result::Error> = diesel::select(exists(
        users.filter(crate::db::schema::users::dsl::email.eq(&e)),
    ))
    .get_result::<bool>(conn);

    match user_exists {
        Ok(result) => Some(result),
        Err(_) => panic!(),
    }
}

pub fn create_user(conn: &mut SqliteConnection, username: &str, e: &str) -> User {
    use crate::db::schema::users;
    use diesel::SelectableHelper;

    let new_user = NewUser { username, email: e };

    diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(conn)
        .expect("err")
}

pub fn delete_user(
    conn: &mut SqliteConnection,
    user: User,
) -> Result<usize, diesel::result::Error> {
    use crate::db::schema::users::dsl::users;

    diesel::delete(users.filter(email.eq(user.email))).execute(conn)
}

pub fn list_users(conn: &mut SqliteConnection) -> Result<Vec<String>, diesel::result::Error> {
    use crate::db::schema::users::dsl::users;
    users.select(email).load::<String>(conn)
}
