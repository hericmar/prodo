use crate::core::services::person::PersonService;
use crate::error::{Error, ErrorType};
use crate::prelude::*;
use actix_identity::error::{GetIdentityError, LoginError};
use actix_identity::Identity;
use actix_web::http::StatusCode;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, ResponseError};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

impl From<LoginError> for Error {
    fn from(_value: LoginError) -> Self {
        Error::new("invalid cookie", ErrorType::Unauthorized)
    }
}

impl From<GetIdentityError> for Error {
    fn from(_value: GetIdentityError) -> Self {
        Error::new("unauthorized", ErrorType::Unauthorized)
    }
}

pub async fn login(
    person_service: web::Data<dyn PersonService>,
    req: HttpRequest,
    payload: web::Json<LoginRequest>,
) -> Result<HttpResponse> {
    let person = person_service
        .get_by_username(&payload.username)
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
    user: Identity,
) -> Result<HttpResponse> {
    let uid: Uuid = user.id()?.parse().unwrap();
    let person = person_service.get(uid).await?;

    Ok(HttpResponse::Ok().json(UserResponse {
        uid: person.uid,
        username: person.username,
    }))
}

pub async fn logout(user: Identity) -> Result<HttpResponse> {
    user.logout();
    Ok(HttpResponse::Ok().finish())
}
