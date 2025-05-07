use crate::schema::schema::pets;
use diesel::{Associations, Identifiable, Insertable, Queryable, Selectable};
use crate::models::user::User;

#[derive(Queryable, Selectable, Identifiable, Associations)]
#[derive(serde::Serialize)]
#[diesel(table_name = pets)]
#[diesel(belongs_to(User, foreign_key = tutor_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Pet {
    pub id: i32, // i64 if bigint
    pub name: String,
    pub description: String,
    pub tutor_id: i32
}

#[derive(Insertable)]
#[diesel(table_name = pets)]
pub struct NewPet {
    pub name: String,
    pub description: String,
    pub tutor_id: i32
}