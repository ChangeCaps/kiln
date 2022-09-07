use std::borrow::Cow;

use crate::Span;

#[derive(Clone, Debug)]
pub struct Error {
    message: Option<Cow<'static, str>>,
    span: Span,
}

impl Error {
    pub fn new(span: Span) -> Self {
        Self {
            message: None,
            span,
        }
    }

    pub fn with_message(mut self, message: impl Into<Cow<'static, str>>) -> Self {
        self.message = Some(message.into());
        self
    }
}

#[derive(Clone, Debug)]
pub struct ErrorHint {}

pub type Result<T> = std::result::Result<T, Error>;
