// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2020-2024 Martin Herich <martin.herich@phire.cz>

use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Error {
    pub what: String,
    pub code: ErrorType
}

#[derive(Debug)]
pub enum ErrorType {
    DatabaseError,
    InternalError,
    NotFound,
    BadRequest,
    Unauthorized
}

impl Error {
    pub fn new(what: &str, code: ErrorType) -> Self {
        Error{
            what: what.to_string(),
            code
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
