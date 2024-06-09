use diesel::r2d2;
use diesel::r2d2::ConnectionManager;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub type Pool<T> = r2d2::Pool<ConnectionManager<T>>;
pub type PostgresPool = Pool<diesel::pg::PgConnection>;
pub type DBPool = PostgresPool;

// This macro from `diesel_migrations` defines an `embedded_migrations` module
// containing a function named `run`. This allows the example to be run and
// tested without any outside setup of the database.
const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub fn create_pool(database_url: &str) -> PostgresPool {
    let manager = ConnectionManager::<diesel::pg::PgConnection>::new(database_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let mut conn = pool.get().unwrap();
    conn.run_pending_migrations(MIGRATIONS).unwrap();

    pool
}
