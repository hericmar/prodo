use crate::error::{Error, ErrorType};
use rrule::ValidationError;
use validator::ValidationErrors;

pub mod calendar;
pub mod person;
pub mod task;

impl From<ValidationError> for Error {
    fn from(value: ValidationError) -> Self {
        Error::new(&value.to_string(), ErrorType::BadRequest)
    }
}

impl From<ValidationErrors> for Error {
    fn from(value: ValidationErrors) -> Self {
        Error::new(&value.to_string(), ErrorType::BadRequest)
    }
}
