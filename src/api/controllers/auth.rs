use actix_identity::error::{GetIdentityError, LoginError};
use actix_identity::Identity;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, ResponseError, web};
use actix_web::http::StatusCode;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::core::services::person::PersonService;
use crate::prelude::*;
use crate::error::{Error, ErrorType};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

impl From<LoginError> for Error {
    fn from(value: LoginError) -> Self {
        Error::new("invalid cookie", ErrorType::Unauthorized)
    }
}

impl From<GetIdentityError> for Error {
    fn from(value: GetIdentityError) -> Self {
        Error::new("unauthorized", ErrorType::Unauthorized)
    }
}

pub async fn login(
    person_service: web::Data<dyn PersonService>,
    req: HttpRequest,
    payload: web::Json<LoginRequest>
) -> Result<HttpResponse> {
    let person = person_service.get_by_username(&payload.username)
        .await
        .map_err(|err| {
            if err.status_code() == StatusCode::NOT_FOUND {
                return Error::new("unauthorized", ErrorType::Unauthorized);
            }
            err
        })?;

    let parsed_hash = PasswordHash::new(&person.password)?;
    let result = Argon2::default().verify_password(payload.password.as_ref(), &parsed_hash);

    match result {
        Ok(_) => {}
        Err(err) => {
            return Err(Error::new(&err.to_string(), ErrorType::Unauthorized));
        }
    }

    if result.is_ok() {
        Identity::login(&req.extensions(), person.uid.to_string())?;
        Ok(HttpResponse::Ok().finish())
    } else {
        Err(Error::new("unauthorized", ErrorType::Unauthorized))
    }
}

#[derive(Serialize)]
pub struct UserResponse {
    pub uid: Uuid,
    pub username: String,
}

pub async fn user(
    person_service: web::Data<dyn PersonService>,
    user: Identity
) -> Result<HttpResponse> {
    let username = user.id().unwrap();
    let person = person_service.get_by_username(&username).await?;

    Ok(HttpResponse::Ok().json(UserResponse {
        uid: person.uid,
        username: person.username
    }))
}

pub async fn logout(
    user: Identity
) -> Result<HttpResponse> {
    user.logout();
    Ok(HttpResponse::Ok().finish())
}