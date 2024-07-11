use crate::core::models::task::URGENCY_HIGH;
use crate::core::services::calendar::CalendarService;
use crate::core::services::task::{into_rrule_datetime, into_rrule_set, TaskService};
use crate::prelude::*;
use crate::utils::time::to_day_bounds;
use actix_identity::Identity;
use actix_web::{web, HttpResponse};
use chrono::{Duration, Utc};

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
    let now_rrule = into_rrule_datetime(Utc::now());

    let mut body = String::new();
    body.push_str("BEGIN:VCALENDAR\n");
    body.push_str(&format!(
        "PRODID:-//Phire//Prodo Calendar {}//EN\n",
        PRODO_VERSION
    ));
    body.push_str("VERSION:2.0\n");
    for task in tasks {
        // export precondition: task has a start time, end time, or due time or is recurring.
        if task.rrule.is_none()
            && (task.dtstart.is_none() || task.dtend.is_none())
            && task.due.is_none()
        {
            continue;
        }

        let (day_start, day_end) = to_day_bounds(&task.created);

        // Start time is task dtstart or due - 12 hours, midnight if not set.
        let mut dtstart = task.dtstart.unwrap_or(day_start);
        if let Some(due) = task.due {
            dtstart = due - Duration::hours(12);
        }

        // End time is task dtend or due, midnight if not set.
        let dtend = task.dtend.unwrap_or(task.due.unwrap_or(day_end));

        // TODO: Implement recurrence rule support.
        let mut missed = false;
        if let Some(rrule) = &task.rrule {
            let rrule_set = into_rrule_set(dtstart, rrule).unwrap();
            let recurrence_start = task.completed.unwrap_or(task.created);
            let recurrences = rrule_set
                .after(into_rrule_datetime(recurrence_start))
                .all(1)
                .dates;
            // TODO
            missed = !recurrences.is_empty() && recurrences[0] < now_rrule;
        }

        let status = if task.completed.is_some() && task.rrule.is_none() {
            "CANCELLED"
        } else {
            "CONFIRMED"
        };

        body.push_str(&format!(
            "BEGIN:VEVENT
UID:{}
DTSTAMP:{}
DTSTART:{}
DTEND:{}
SUMMARY:{}
DESCRIPTION:{}
STATUS:{}
SEQUENCE:{}\n",
            task.uid,
            task.created.format("%Y%m%dT%H%M%SZ"),
            dtstart.format("%Y%m%dT%H%M%SZ"),
            dtend.format("%Y%m%dT%H%M%SZ"),
            task.summary,
            task.description,
            status,
            task.sequence
        ));
        if let Some(rrule) = &task.rrule {
            body.push_str(&format!("{}\n", rrule));
        }
        if task.due.is_some() {
            // A trigger set 15 minutes prior to the start of the event or
            // to-do.
            //
            //   TRIGGER:-P15M
            //
            // A trigger set 5 minutes after the end of the event or to-do.
            //
            //   TRIGGER;RELATED=END:P5M
            body.push_str(
                &"BEGIN:VALARM
ACTION:DISPLAY
TRIGGER:-P1H
DESCRIPTION:This is an event reminder
END:VALARM\n"
                    .to_string(),
            );
        }

        body.push_str("END:VEVENT\n")
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
