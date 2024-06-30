#[macro_use]
extern crate diesel_migrations;

mod api;
mod bootstrap;
mod core;
mod error;
mod infrastructure;
mod prelude;
mod schema;
mod services;
mod tests;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    bootstrap::start()
        .await
        .unwrap_or_else(|err| eprintln!("Error: {}", err));

    Ok(())
}
