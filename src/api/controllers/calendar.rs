use crate::core::services::calendar::CalendarService;
use crate::core::services::task::TaskService;
use crate::prelude::*;
use actix_identity::Identity;
use actix_web::{web, HttpResponse};
use chrono::Duration;
use icalendar::{
    Alarm, Calendar, Component, Event, EventLike, EventStatus, Property, Related, Trigger,
};

const PRODO_VERSION: &str = env!("CARGO_PKG_VERSION");

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

    let mut calendar = Calendar::new()
        .append_property(Property::new(
            "PRODID",
            &format!("-//Prodo//Prodo Calendar {}//EN", PRODO_VERSION),
        ))
        .timezone("Europe/Prague")
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

    let response = HttpResponse::Ok()
        .insert_header(("Content-Type", "text/calendar; charset=utf-8"))
        .body(calendar.to_string());

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
