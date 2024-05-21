// @generated automatically by Diesel CLI.

diesel::table! {
    auth_group (id) {
        id -> Int4,
        #[max_length = 150]
        name -> Varchar,
    }
}

diesel::table! {
    auth_group_permissions (id) {
        id -> Int8,
        group_id -> Int4,
        permission_id -> Int4,
    }
}

diesel::table! {
    auth_permission (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        content_type_id -> Int4,
        #[max_length = 100]
        codename -> Varchar,
    }
}

diesel::table! {
    auth_user (id) {
        id -> Int4,
        #[max_length = 128]
        password -> Varchar,
        last_login -> Nullable<Timestamptz>,
        is_superuser -> Bool,
        #[max_length = 150]
        username -> Varchar,
        #[max_length = 150]
        first_name -> Varchar,
        #[max_length = 150]
        last_name -> Varchar,
        #[max_length = 254]
        email -> Varchar,
        is_staff -> Bool,
        is_active -> Bool,
        date_joined -> Timestamptz,
    }
}

diesel::table! {
    auth_user_groups (id) {
        id -> Int8,
        user_id -> Int4,
        group_id -> Int4,
    }
}

diesel::table! {
    auth_user_user_permissions (id) {
        id -> Int8,
        user_id -> Int4,
        permission_id -> Int4,
    }
}

diesel::table! {
    django_admin_log (id) {
        id -> Int4,
        action_time -> Timestamptz,
        object_id -> Nullable<Text>,
        #[max_length = 200]
        object_repr -> Varchar,
        action_flag -> Int2,
        change_message -> Text,
        content_type_id -> Nullable<Int4>,
        user_id -> Int4,
    }
}

diesel::table! {
    django_content_type (id) {
        id -> Int4,
        #[max_length = 100]
        app_label -> Varchar,
        #[max_length = 100]
        model -> Varchar,
    }
}

diesel::table! {
    django_cron_cronjoblock (id) {
        id -> Int4,
        #[max_length = 200]
        job_name -> Varchar,
        locked -> Bool,
    }
}

diesel::table! {
    django_cron_cronjoblog (id) {
        id -> Int4,
        #[max_length = 64]
        code -> Varchar,
        start_time -> Timestamptz,
        end_time -> Timestamptz,
        is_success -> Bool,
        message -> Text,
        ran_at_time -> Nullable<Time>,
    }
}

diesel::table! {
    django_migrations (id) {
        id -> Int8,
        #[max_length = 255]
        app -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        applied -> Timestamptz,
    }
}

diesel::table! {
    django_session (session_key) {
        #[max_length = 40]
        session_key -> Varchar,
        session_data -> Text,
        expire_date -> Timestamptz,
    }
}

diesel::table! {
    ical_subscription (id) {
        id -> Int8,
        created -> Timestamptz,
        #[max_length = 64]
        secret -> Varchar,
        created_by_id -> Int4,
        user_id -> Int4,
    }
}

diesel::table! {
    persons (id) {
        id -> Int4,
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
    tasks_task (uid) {
        created -> Timestamptz,
        updated -> Timestamptz,
        uid -> Uuid,
        #[max_length = 60]
        summary -> Varchar,
        description -> Text,
        start -> Nullable<Timestamptz>,
        end -> Nullable<Timestamptz>,
        due -> Nullable<Timestamptz>,
        #[max_length = 255]
        rrule -> Nullable<Varchar>,
        sequence -> Int4,
        active -> Bool,
        created_by_id -> Int4,
        updated_by_id -> Nullable<Int4>,
        completed -> Nullable<Timestamptz>,
        priority -> Int2,
        score -> Float8,
        urgency -> Int2,
    }
}

diesel::table! {
    tasks_taskevent (id) {
        id -> Int8,
        created -> Timestamptz,
        event_type -> Int4,
        created_by_id -> Int4,
        task_id -> Uuid,
    }
}

diesel::table! {
    tasks_tasklist (id) {
        id -> Int8,
        created -> Timestamptz,
        ordered_tasks -> Jsonb,
        created_by_id -> Int4,
        user_id -> Int4,
    }
}

diesel::table! {
    token_blacklist_blacklistedtoken (id) {
        id -> Int8,
        blacklisted_at -> Timestamptz,
        token_id -> Int8,
    }
}

diesel::table! {
    token_blacklist_outstandingtoken (id) {
        id -> Int8,
        token -> Text,
        created_at -> Nullable<Timestamptz>,
        expires_at -> Timestamptz,
        user_id -> Nullable<Int4>,
        #[max_length = 255]
        jti -> Varchar,
    }
}

diesel::table! {
    users_userdata (id) {
        id -> Int8,
        urgency_calculated_at -> Nullable<Timestamptz>,
        user_id -> Int4,
    }
}

diesel::joinable!(auth_group_permissions -> auth_group (group_id));
diesel::joinable!(auth_group_permissions -> auth_permission (permission_id));
diesel::joinable!(auth_permission -> django_content_type (content_type_id));
diesel::joinable!(auth_user_groups -> auth_group (group_id));
diesel::joinable!(auth_user_groups -> auth_user (user_id));
diesel::joinable!(auth_user_user_permissions -> auth_permission (permission_id));
diesel::joinable!(auth_user_user_permissions -> auth_user (user_id));
diesel::joinable!(django_admin_log -> auth_user (user_id));
diesel::joinable!(django_admin_log -> django_content_type (content_type_id));
diesel::joinable!(tasks_taskevent -> auth_user (created_by_id));
diesel::joinable!(tasks_taskevent -> tasks_task (task_id));
diesel::joinable!(token_blacklist_blacklistedtoken -> token_blacklist_outstandingtoken (token_id));
diesel::joinable!(token_blacklist_outstandingtoken -> auth_user (user_id));
diesel::joinable!(users_userdata -> auth_user (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    auth_group,
    auth_group_permissions,
    auth_permission,
    auth_user,
    auth_user_groups,
    auth_user_user_permissions,
    django_admin_log,
    django_content_type,
    django_cron_cronjoblock,
    django_cron_cronjoblog,
    django_migrations,
    django_session,
    ical_subscription,
    persons,
    tasks_task,
    tasks_taskevent,
    tasks_tasklist,
    token_blacklist_blacklistedtoken,
    token_blacklist_outstandingtoken,
    users_userdata,
);
