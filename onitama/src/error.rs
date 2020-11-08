use crate::messages::LitamaMessage;
use std::error::Error;
use std::fmt;

// TODO Put some effort into error management instead of boxing everything
pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct UnexpectedMessage {
    message: LitamaMessage,
}

impl UnexpectedMessage {
    pub fn new(message: LitamaMessage) -> UnexpectedMessage {
        UnexpectedMessage { message }
    }
}

impl fmt::Display for UnexpectedMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unexpected message: {:#?}", self.message)
    }
}

impl Error for UnexpectedMessage {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[derive(Debug)]
pub struct UnexpectedVariant {
    variant: String,
}

impl UnexpectedVariant {
    pub fn new(variant: String) -> UnexpectedVariant {
        UnexpectedVariant { variant }
    }
}

impl fmt::Display for UnexpectedVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unexpected variant: {}", self.variant)
    }
}

impl Error for UnexpectedVariant {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
