use crate::core::services::calendar::CalendarService;
use crate::core::services::task::TaskService;
use crate::prelude::*;
use actix_identity::Identity;
use actix_web::http::header::HeaderName;
use actix_web::{web, HttpRequest, HttpResponse};
use chrono::Duration;
use std::fmt::Write;

pub async fn create_calendar_subscription_handler(
    user: Identity,
    calendar_service: web::Data<dyn CalendarService>,
) -> Result<HttpResponse> {
    let subscription = calendar_service
        .create_subscription(user.id()?.parse().unwrap())
        .await?;

    Ok(HttpResponse::Ok().json(subscription))
}

pub async fn get_calendar_subscription_handler(
    calendar_service: web::Data<dyn CalendarService>,
    task_service: web::Data<dyn TaskService>,
    secret: web::Path<String>,
) -> Result<HttpResponse> {
    let subscription = calendar_service
        .get_subscription(secret.into_inner())
        .await?;

    let tasks = task_service.list(subscription.person_uid).await?;

    let mut body = String::new();
    body.push_str("BEGIN:VCALENDAR\n");
    body.push_str("PRODID:-//Prodo//Prodo Calendar//EN\n");
    body.push_str("VERSION:2.0.0\n");
    for task in tasks {
        let end = task.created + Duration::hours(2);

        body.push_str(&format!(
            "BEGIN:VEVENT
UID:{}
DTSTAMP:{}
DTSTART:{}
DTEND:{}
SUMMARY:{}
DESCRIPTION:{}
END:VEVENT\n",
            task.uid,
            task.created.format("%Y%m%dT%H%M%SZ"),
            task.created.format("%Y%m%dT%H%M%SZ"),
            end.format("%Y%m%dT%H%M%SZ"),
            task.summary,
            task.description
        ));
    }
    body.push_str("END:VCALENDAR\n");
    body = body.replace("\n", "\r\n");

    let response = HttpResponse::Ok()
        .insert_header(("Content-Type", "text/calendar; charset=utf-8"))
        .body(body);

    Ok(response)
}

pub async fn get_my_calendar_subscription_handler(
    user: Identity,
    calendar_service: web::Data<dyn CalendarService>,
) -> Result<HttpResponse> {
    let subscription = calendar_service
        .get_person_subscription(user.id()?.parse().unwrap())
        .await?;

    Ok(HttpResponse::Ok().json(subscription))
}

pub async fn delete_my_calendar_subscription_handler(
    user: Identity,
    calendar_service: web::Data<dyn CalendarService>,
) -> Result<HttpResponse> {
    calendar_service
        .delete_subscription(user.id()?.parse().unwrap())
        .await?;

    Ok(HttpResponse::Ok().finish())
}
