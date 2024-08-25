use crate::core::models::calendar::UpdateCalendarSubscription;
use crate::core::services::calendar::CalendarService;
use crate::core::services::task::TaskService;
use crate::prelude::*;
use actix_identity::Identity;
use actix_web::{web, HttpResponse};
use chrono::{Duration, Utc};
use icalendar::{
    Alarm, Calendar, Component, Event, EventLike, EventStatus, Property, Related, Trigger,
};
use serde::Deserialize;

const PRODO_VERSION: &str = env!("CARGO_PKG_VERSION");

pub async fn get_calendar_handler(
    calendar_service: web::Data<dyn CalendarService>,
    task_service: web::Data<dyn TaskService>,
    secret: web::Path<String>,
) -> Result<HttpResponse> {
    let subscription = calendar_service
        .get_subscription(secret.into_inner())
        .await?;

    let tasks = task_service.list(subscription.person_uid).await?;

    let mut calendar = Calendar::new()
        .append_property(Property::new(
            "PRODID",
            &format!("-//Prodo//Prodo Calendar {}//EN", PRODO_VERSION),
        ))
        .timezone(&subscription.timezone)
        .done();

    for task in tasks {
        if task.rrule.is_none()
            && (task.dtstart.is_none() || task.dtend.is_none())
            && task.due.is_none()
        {
            continue;
        }

        let status: EventStatus = if task.completed.is_some() && task.rrule.is_none() {
            EventStatus::Cancelled
        } else {
            EventStatus::Confirmed
        };

        let mut event = &mut Event::new();
        event = event
            .uid(&task.uid.to_string())
            .timestamp(task.created)
            .summary(&task.summary)
            .description(&task.description)
            .sequence(task.sequence as u32)
            .status(status);

        if (task.due.is_some() || task.rrule.is_some()) && task.dtend.is_none() {
            let date = if let Some(_) = &task.rrule {
                task.dtstart.unwrap_or(task.created)
            } else {
                task.due.unwrap()
            };
            event = event.all_day(date.date_naive())
        } else {
            event = event
                .starts(task.dtstart.unwrap_or(task.created))
                .ends(task.dtend.unwrap_or(task.due.unwrap_or(task.created)))
        }

        // Recurrence rule
        if let Some(rrule) = &task.rrule {
            event = event.add_property("RRULE", rrule);
        }

        // Due date alarm
        if let Some(_) = &task.due {
            event = event.alarm(Alarm::display(
                "This is an event reminder",
                Trigger::Duration(-Duration::hours(1), Some(Related::End)),
            ));
        }

        calendar.push(event.done());
    }

    calendar_service
        .update_last_synced_at(subscription.person_uid, Utc::now())
        .await?;

    let calendar_str = calendar.to_string();
    // Ugly workaround to fix the escaping of the comma and semicolon in RRULE property.
    let calendar_str = calendar_str.lines().fold(String::new(), |mut acc, line| {
        if line.starts_with("RRULE") {
            acc.push_str(line.replace("\\,", ",").replace("\\;", ";").as_str());
        } else {
            acc.push_str(line);
        }
        acc.push_str("\r\n");
        acc
    });

    let response = HttpResponse::Ok()
        .insert_header(("Content-Type", "text/calendar; charset=utf-8"))
        .body(calendar_str);

    Ok(response)
}

#[derive(Deserialize)]
pub struct CreateCalendarSubscription {
    pub timezone: Option<String>,
}

pub async fn create_calendar_subscription_handler(
    user: Identity,
    calendar_service: web::Data<dyn CalendarService>,
    payload: web::Json<CreateCalendarSubscription>,
) -> Result<HttpResponse> {
    let timezone = payload
        .timezone
        .clone()
        .unwrap_or("Europe/London".to_string())
        .clone();

    let subscription = calendar_service
        .create_subscription(user.id()?.parse().unwrap(), &timezone)
        .await?;

    Ok(HttpResponse::Ok().json(subscription))
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

pub async fn update_my_calendar_subscription_handler(
    user: Identity,
    calendar_service: web::Data<dyn CalendarService>,
    payload: web::Json<UpdateCalendarSubscription>,
) -> Result<HttpResponse> {
    let subscription = calendar_service
        .update_subscription(user.id()?.parse().unwrap(), &payload)
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
