use diesel::prelude::*;
use diesel::query_dsl::QueryDsl;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use crate::api::schema::users::dsl::*;
use crate::api::models::User;

use diesel::{SqliteConnection};

pub struct UserService {
    pool: Pool<ConnectionManager<SqliteConnection>>,
}

impl UserService {
    pub fn new(pool: Pool<ConnectionManager<SqliteConnection>>) -> Self {
        UserService { pool }
    }

    fn get_conn(&self) -> PooledConnection<ConnectionManager<SqliteConnection>> {
        self.pool.get().expect("Couldn't get DB connection")
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
}