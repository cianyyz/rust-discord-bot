use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
    num::ParseIntError,
};

#[derive(Debug)]
pub enum BotError {
    ParseError(ParseIntError),
    SerenityError(serenity::Error),
    CustomError(String),
    NoneError,
    LogTypeNotFound,
    LogTypeDisabled,
    NoRecordYet,
    NegativeMoney,
}

impl Display for BotError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut msg = String::from("[BotError] ");
        let error = match self {
            BotError::ParseError(e) => e.to_string(),
            BotError::SerenityError(e) => e.to_string(),
            BotError::CustomError(e) => e.to_string(),
            BotError::NoneError => "Unwrapped a None Option".into(),
            BotError::LogTypeNotFound => "Log type was not found".into(),
            BotError::LogTypeDisabled => "This log type is disabled".into(),
            BotError::NoRecordYet => "User has no score record yet".into(),
            BotError::NegativeMoney => "The amount of money cannot be negative.".into(),
        };
        msg += &error;
        f.write_str(&msg)
    }
}

impl Error for BotError {}

impl From<String> for BotError {
    fn from(err: String) -> BotError {
        BotError::CustomError(err)
    }
}

impl From<ParseIntError> for BotError {
    fn from(err: ParseIntError) -> BotError {
        BotError::ParseError(err)
    }
}

impl From<serenity::Error> for BotError {
    fn from(err: serenity::Error) -> BotError {
        BotError::SerenityError(err)
    }
}