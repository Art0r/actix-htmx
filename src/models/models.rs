use diesel::prelude::*;
use crate::schema::schema::users;

#[derive(Queryable, Selectable)]
#[derive(serde::Serialize)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: i32, // i64 if bigint
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}