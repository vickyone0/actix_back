use std::fmt;

#[derive(Debug)]
pub enum AppError {
    UserNotFound,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::UserNotFound => write!(f, "User not found"),
        }
    }
}

impl std::error::Error for AppError {}