use crate::db::schema::users;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub enabled: bool,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
}

impl User {
    pub fn new(email: &str, username: &str) -> Self {
        Self {
            email: email.to_string(),
            username: username.to_string(),
            id: 0,
            enabled: false,
        }
    }
}
