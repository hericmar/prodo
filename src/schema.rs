// @generated automatically by Diesel CLI.

diesel::table! {
    calendar_subscriptions (secret) {
        #[max_length = 255]
        secret -> Varchar,
        person_uid -> Uuid,
    }
}

diesel::table! {
    lists (uid) {
        uid -> Uuid,
        name -> Text,
        author_uid -> Uuid,
        tasks -> Array<Nullable<Uuid>>,
    }
}

diesel::table! {
    persons (uid) {
        uid -> Uuid,
        #[max_length = 32]
        first_name -> Varchar,
        #[max_length = 32]
        surname -> Varchar,
        #[max_length = 32]
        username -> Varchar,
        #[max_length = 320]
        email -> Varchar,
        password -> Text,
    }
}

diesel::table! {
    tasks (uid) {
        uid -> Uuid,
        #[max_length = 255]
        summary -> Varchar,
        description -> Text,
        completed -> Nullable<Timestamptz>,
        created -> Timestamptz,
        author_uid -> Uuid,
        rrule -> Nullable<Text>,
        dtstart -> Nullable<Timestamptz>,
        dtend -> Nullable<Timestamptz>,
        priority -> Int4,
        due -> Nullable<Timestamptz>,
        sequence -> Int4,
        urgency -> Int4,
    }
}

diesel::joinable!(calendar_subscriptions -> persons (person_uid));
diesel::joinable!(lists -> persons (author_uid));
diesel::joinable!(tasks -> persons (author_uid));

diesel::allow_tables_to_appear_in_same_query!(
    calendar_subscriptions,
    lists,
    persons,
    tasks,
);
