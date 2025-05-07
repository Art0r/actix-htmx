use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{PgConnection};
use tera::Tera;

pub struct AppState {
    pub db: Pool<ConnectionManager<PgConnection>>,
    pub tera: Tera,
}