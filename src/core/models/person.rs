use diesel::prelude::*;
use diesel::{Insertable, Queryable, Selectable};
use crate::schema::persons::dsl::persons;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::persons)]
pub struct Person {
    pub(crate) id: i32,
    first_name: String,
    surname: String,
    pub(crate) username: String,
    pub(crate) email: String,
    password: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::persons)]
pub struct CreatePerson {
    pub first_name: String,
    pub surname: String,
    pub username: String,
    pub email: String,
    pub password: String,
}
