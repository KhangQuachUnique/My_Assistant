use std::{
    fmt::{Display, Formatter},
    path::PathBuf,
};

#[allow(dead_code)]
#[derive(Debug)]
pub enum AvatarError {
    ExeNotFound(PathBuf),
    ProcessStartFailed(String),
    ProcessStopFailed(String),
    ProcessStatusUnavailable(String),
    Busy,
}

impl Display for AvatarError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ExeNotFound(path) => {
                write!(formatter, "avatar executable not found: {}", path.display())
            }
            Self::ProcessStartFailed(message) => {
                write!(formatter, "failed to start avatar process: {message}")
            }
            Self::ProcessStopFailed(message) => {
                write!(formatter, "failed to stop avatar process: {message}")
            }
            Self::ProcessStatusUnavailable(message) => {
                write!(formatter, "avatar process status unavailable: {message}")
            }
            Self::Busy => write!(formatter, "avatar module is busy"),
        }
    }
}

impl std::error::Error for AvatarError {}
