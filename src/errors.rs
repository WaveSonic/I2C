use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Io(std::io::Error),
    CommandError(String),
    EmptyData(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Io(err) => write!(f, "Помилка вводу/виводу: {}", err),
            AppError::CommandError(msg) => write!(f, "Помилка виконання команди: {}", msg),
            AppError::EmptyData(msg) => write!(f, "Порожні дані: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

impl From<std::io::Error> for AppError {
    fn from(value: std::io::Error) -> Self {
        AppError::Io(value)
    }
}