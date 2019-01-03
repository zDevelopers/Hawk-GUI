use std::result::Result;

use uuid::Uuid;

pub type ReportResult<T> = Result<T, InvalidReportError>;

#[derive(Debug, Fail, AsRefStr)]
pub enum InvalidReportError {
    #[fail(display = "No player with this UUID can be found in the players list: {}", uuid)]
    MissingPlayerReference { uuid: Uuid },

    #[fail(display = "An unknown error happened")]
    Unknown
}
