use crate::infrastructure::cli;

mod bootstrap;
mod core;
mod api;
mod prelude;
mod error;
mod services;
mod schema;
mod tests;
mod infrastructure;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    bootstrap::start()
        .await
        .unwrap_or_else(|err| eprintln!("Error: {}", err));

    Ok(())
}
