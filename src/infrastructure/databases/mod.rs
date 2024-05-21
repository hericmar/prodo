use r2d2;
use diesel::result::DatabaseErrorKind;
use crate::error::{Error, ErrorType};

pub mod postgres;

impl From<r2d2::Error> for Error {
    fn from(err: r2d2::Error) -> Self {
        Error::new(&err.to_string(), ErrorType::DatabaseError)
    }
}

impl From<diesel::result::Error> for Error {
    fn from(e: diesel::result::Error) -> Self {
        return match e {
            diesel::result::Error::DatabaseError(kind, info) => {
                let error_type = match kind {
                    DatabaseErrorKind::UniqueViolation |
                    DatabaseErrorKind::ForeignKeyViolation |
                    DatabaseErrorKind::NotNullViolation |
                    DatabaseErrorKind::CheckViolation => {

                        ErrorType::BadRequest
                    }
                    _ => {
                        ErrorType::DatabaseError
                    }
                };
                Error::new(&format!("Database query error: {}", info.message()), error_type)
            }
            diesel::result::Error::NotFound => {
                Error::new("Record not found", ErrorType::NotFound)
            }
            _ => {
                Error::new(&format!("Database query error: {}", e), ErrorType::DatabaseError)
            }
        }
    }
}
