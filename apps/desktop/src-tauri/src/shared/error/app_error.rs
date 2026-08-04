use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum AppError {
    Configuration(String),
    Initialization(String),
    Lifecycle(String),
    Internal(String),
}

impl Display for AppError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Configuration(message) => {
                write!(formatter, "configuration error: {message}")
            }
            Self::Initialization(message) => {
                write!(formatter, "initialization error: {message}")
            }
            Self::Lifecycle(message) => {
                write!(formatter, "application lifecycle error: {message}")
            }
            Self::Internal(message) => {
                write!(formatter, "internal error: {message}")
            }
        }
    }
}

impl std::error::Error for AppError {}
