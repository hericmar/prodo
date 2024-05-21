use std::io;
use std::io::Write;
use std::sync::Arc;
use actix_web::{App, HttpServer, web};
use clap::Parser;
use log::error;
use rpassword::read_password;
use crate::prelude::*;
use crate::api::controllers::task::read_tasks_handler;
use crate::core::models::person::{CreatePerson, Person};
use crate::core::repositories::person::PersonRepository;
use crate::core::repositories::task::TaskRepository;
use crate::core::services::person::PersonService;
use crate::error::{Error, ErrorType};
use crate::infrastructure::cli::{Cli, Commands, UserCommands};
use crate::infrastructure::databases::postgres;
use crate::infrastructure::repositories::person::PersonRepositoryImpl;
use crate::services::person::PersonServiceImpl;
use crate::services::task::TaskServiceImpl;
fn setup(app: &mut web::ServiceConfig) {
    app.service(
        web::scope("/api/v1")
            .service(
                web::scope("/tasks")
                    .route("", web::get().to(read_tasks_handler))
            )
    );
}

pub async fn start() -> Result<()> {
    let db_pool = Arc::new(
        postgres::create_pool("postgres://prodo:prodo@localhost:5432/prodo"));

    let person_repository: Arc<dyn PersonRepository> = Arc::new(PersonRepositoryImpl::new(db_pool.clone()));
    let person_service = Arc::new(PersonServiceImpl::new(person_repository));

    match Cli::parse().command {
        Commands::User(parent) => match &parent.command {
            UserCommands::Create(command) => {
                print!("Enter password: ");
                std::io::stdout().flush().expect("Failed to flush stdout");
                let password = read_password().expect("Failed to read password");
                print!("Confirm password: ");
                std::io::stdout().flush().expect("Failed to flush stdout");
                let confirm_password = read_password().expect("Failed to read password");

                if password != confirm_password || password.is_empty() {
                    return Err(Error::new("Passwords do not match", ErrorType::BadRequest));
                }

                let person = CreatePerson {
                    first_name: "".to_string(),
                    surname: "".to_string(),
                    username: command.username.clone(),
                    email: "".to_string(),
                    password: "".to_string(),
                };
                person_service.create(person).await?;
                println!("User created.");
            }
            UserCommands::List => {
                let persons = person_service.list().await?;
                for person in persons {
                    println!("{} <{}>", person.username, person.email);
                }
            }
            UserCommands::Delete(command) => {
                let person = person_service.get_by_username(&command.username).await?;
                person_service.delete(person.id).await?;
                println!("User deleted.");
            }
        },
        Commands::Start => {
            HttpServer::new(move || {
                /*
                let task_repository: Arc<dyn TaskRepository> = Arc::new(TaskRepositoryImpl::new());
                let task_service = TaskServiceImpl::new();
                 */

                App::new()
                    .app_data(web::Data::from(person_service.clone()))
                    .configure(setup)
            })
                .bind("0.0.0.0:8080")
                .unwrap_or_else(|err| panic!("Could not bind server: {}", err))
                .run()
                .await
                .unwrap_or_else(|err| error!("Server error: {}", err));
        }
    }

    Ok(())
}
