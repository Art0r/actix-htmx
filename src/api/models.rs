use diesel::prelude::*;
use crate::api::schema::users;

#[derive(Queryable, Selectable)]
#[derive(serde::Serialize)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: i32, // i64 if bigint
    pub name: String,
    pub email: String,
}