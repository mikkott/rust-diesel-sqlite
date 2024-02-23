use crate::db::models::NewUser;
use crate::db::models::User;
use diesel::SqliteConnection;

pub fn exist_user(conn: &mut SqliteConnection, e: &str) -> Option<bool> {
    use crate::db::schema::users::dsl::*;
    use diesel::dsl::exists;
    use diesel::query_dsl::methods::FilterDsl;
    use diesel::ExpressionMethods;
    use diesel::RunQueryDsl;

    let user_exists: Result<bool, diesel::result::Error> = diesel::select(exists(
        users.filter(crate::db::schema::users::dsl::email.eq(&e)),
    ))
    .get_result::<bool>(conn);

    match user_exists {
        Ok(result) => Some(result),
        Err(_) => panic!(),
    }
}

pub fn create_user(conn: &mut SqliteConnection, username: &str, email: &str) -> User {
    use crate::db::schema::users;
    use diesel::RunQueryDsl;
    use diesel::SelectableHelper;

    let new_user = NewUser { username, email };

    diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(conn)
        .expect("err")
}
