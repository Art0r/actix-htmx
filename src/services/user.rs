use actix_web::web;
use diesel::prelude::*;
use diesel::query_dsl::QueryDsl;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use crate::schema::schema::users::dsl::*;

use diesel::{PgConnection};
use diesel::associations::HasTable;
use crate::errors::user::UserError;
use crate::forms::user::{CreateUserForm, EditUserForm};
use crate::models::pet::Pet;
use crate::models::user::{NewUser, User};
use crate::schema::schema::pets::dsl::pets;

pub struct UserService {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl UserService {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        UserService { pool }
    }

    fn get_conn(&self) -> PooledConnection<ConnectionManager<PgConnection>> {
        self.pool.get().expect("Couldn't get DB connection")
    }

    pub fn edit_user(&self, uid: web::Path<i32>, form: EditUserForm) -> Result<User, UserError> {
        let mut conn = self.get_conn();

        diesel::update(users)
            .filter(id.eq(uid.into_inner()))
            .set((
                name.eq(form.name),
                email.eq(form.email),
                password.eq(form.password),
            ))
            .get_result(&mut conn)
            .map_err(UserError::from)
    }

    pub fn sign_up(&self, form: CreateUserForm) -> Result<User, UserError> {
        let mut conn = self.get_conn();

        let new_user = NewUser {
            name: form.name.to_string(),
            email: form.email.to_string(),
            password: form.password.to_string(),
        };

        diesel::insert_into(users::table())
            .values(&new_user)
            .returning(User::as_returning())
            .get_result(&mut conn)
            .map_err(UserError::from)
    }

    pub fn find_all(&self) -> QueryResult<Vec<User>> {
        let mut conn = self.get_conn();
        users.load::<User>(&mut conn)
    }

    pub fn find_one(&self, uid: i32) -> QueryResult<Option<User>> {
        let mut conn = self.get_conn();
        users.filter(id.eq(uid))
            .first(&mut conn)
            .optional()
    }

    pub fn delete(&self, user_id: i32) -> Result<User, UserError> {
        let mut conn = self.get_conn();

        diesel::delete(users::table())
            .filter(id.eq(user_id))
            .returning(User::as_returning())
            .get_result(&mut conn)
            .map_err(UserError::from)
    }
}