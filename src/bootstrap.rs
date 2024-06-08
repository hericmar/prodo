use crate::api::controllers::auth;
use crate::api::controllers::calendar::{
    create_calendar_subscription_handler, delete_my_calendar_subscription_handler,
    get_calendar_subscription_handler, get_my_calendar_subscription_handler,
};
use crate::api::controllers::task::{
    create_task_handler, delete_task_handler, read_task_lists_handler, read_tasks_handler,
    update_task_handler, update_task_position_handler,
};
use crate::core::models::person::{CreatePerson, Person};
use crate::core::repositories::calendar::CalendarSubscriptionRepository;
use crate::core::repositories::person::PersonRepository;
use crate::core::repositories::task::{TaskListRepository, TaskRepository};
use crate::core::services::calendar::CalendarService;
use crate::core::services::person::PersonService;
use crate::core::services::task::{TaskListService, TaskService};
use crate::error::{Error, ErrorType};
use crate::infrastructure::cli::{Cli, Commands, UserCommands};
use crate::infrastructure::databases::postgres;
use crate::infrastructure::repositories::calendar::CalendarSubscriptionRepositoryImpl;
use crate::infrastructure::repositories::person::PersonRepositoryImpl;
use crate::infrastructure::repositories::task::{TaskListRepositoryImpl, TaskRepositoryImpl};
use crate::prelude::*;
use crate::services::calendar::CalendarServiceImpl;
use crate::services::person::PersonServiceImpl;
use crate::services::task::{TaskListServiceImpl, TaskServiceImpl};
use actix_identity::IdentityMiddleware;
use actix_session::config::PersistentSession;
use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::time::Duration;
use actix_web::cookie::Key;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use clap::Parser;
use log::error;
use rpassword::read_password;
use std::io::Write;
use std::sync::Arc;

fn setup(app: &mut web::ServiceConfig) {
    app.service(
        web::scope("/api/v1")
            .service(
                web::scope("/auth")
                    .route("/login", web::post().to(auth::login))
                    .route("/user", web::get().to(auth::user))
                    .route("/logout", web::post().to(auth::logout)),
            )
            .service(
                web::scope("/tasks")
                    .route("", web::get().to(read_tasks_handler))
                    .route("/{task_uid}", web::patch().to(update_task_handler))
                    .route("/{task_uid}", web::delete().to(delete_task_handler)),
            )
            .service(
                web::scope("/lists")
                    .route("", web::get().to(read_task_lists_handler))
                    .route("/{list_uid}", web::patch().to(read_task_lists_handler))
                    .route("/{list_uid}/tasks", web::post().to(create_task_handler))
                    .route(
                        "/{list_uid}/tasks/{task_uid}/position",
                        web::put().to(update_task_position_handler),
                    ),
            )
            .service(
                web::scope("/calendar")
                    .route(
                        "/subscription/{secret}",
                        web::get().to(get_calendar_subscription_handler),
                    )
                    .route(
                        "/subscription",
                        web::post().to(create_calendar_subscription_handler),
                    )
                    .route(
                        "/subscription",
                        web::get().to(get_my_calendar_subscription_handler),
                    )
                    .route(
                        "/subscription",
                        web::delete().to(delete_my_calendar_subscription_handler),
                    ),
            ),
    );
}

pub async fn start() -> Result<()> {
    let db_pool = Arc::new(postgres::create_pool(
        "postgres://prodo:prodo@localhost:5432/prodo",
    ));

    let task_list_repository: Arc<dyn TaskListRepository> =
        Arc::new(TaskListRepositoryImpl::new(db_pool.clone()));
    let task_list_service: Arc<dyn TaskListService> =
        Arc::new(TaskListServiceImpl::new(task_list_repository.clone()));

    let person_repository: Arc<dyn PersonRepository> =
        Arc::new(PersonRepositoryImpl::new(db_pool.clone()));
    let person_service: Arc<dyn PersonService> = Arc::new(PersonServiceImpl::new(
        person_repository,
        task_list_repository.clone(),
    ));

    let task_repository: Arc<dyn TaskRepository> =
        Arc::new(TaskRepositoryImpl::new(db_pool.clone()));
    let task_service: Arc<dyn TaskService> =
        Arc::new(TaskServiceImpl::new(task_repository, task_list_repository));

    let calendar_subscription_repository: Arc<dyn CalendarSubscriptionRepository> =
        Arc::new(CalendarSubscriptionRepositoryImpl::new(db_pool.clone()));
    let calendar_service: Arc<dyn CalendarService> =
        Arc::new(CalendarServiceImpl::new(calendar_subscription_repository));

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
                    uid: None,
                    first_name: "".to_string(),
                    surname: "".to_string(),
                    username: command.username.clone(),
                    email: "".to_string(),
                    password,
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
                person_service.delete(person.uid).await?;
                println!("User deleted.");
            }
        },
        Commands::Start => {
            let secret_key = Key::generate();

            HttpServer::new(move || {
                /*
                let task_repository: Arc<dyn TaskRepository> = Arc::new(TaskRepositoryImpl::new());
                let task_service = TaskServiceImpl::new();
                 */

                App::new()
                    .app_data(web::Data::from(person_service.clone()))
                    .app_data(web::Data::from(task_service.clone()))
                    .app_data(web::Data::from(task_list_service.clone()))
                    .app_data(web::Data::from(calendar_service.clone()))
                    .wrap(Logger::default())
                    .wrap(IdentityMiddleware::default())
                    .wrap(
                        SessionMiddleware::builder(
                            CookieSessionStore::default(),
                            secret_key.clone(),
                        )
                        .cookie_name("session".to_owned())
                        .session_lifecycle(
                            PersistentSession::default().session_ttl(Duration::hours(24 * 7)),
                        )
                        .build(),
                    )
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