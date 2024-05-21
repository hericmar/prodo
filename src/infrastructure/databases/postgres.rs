use diesel::r2d2;
use diesel::r2d2::ConnectionManager;
use crate::error::{Error, ErrorType};

pub type Pool<T> = r2d2::Pool<ConnectionManager<T>>;
pub type PostgresPool = Pool<diesel::pg::PgConnection>;
pub type DBPool = PostgresPool;

pub fn create_pool(database_url: &str) -> PostgresPool {
    let manager = ConnectionManager::<diesel::pg::PgConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}
