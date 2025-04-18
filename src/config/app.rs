use diesel::r2d2::{ConnectionManager, Pool};
use diesel::SqliteConnection;
use tera::Tera;

pub struct AppState {
    pub db: Pool<ConnectionManager<SqliteConnection>>,
    pub tera: Tera,
}