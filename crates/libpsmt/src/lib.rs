use std::process::{Termination, ExitCode};

pub use libpsmt_impl::*;

pub trait ExecutableCommand {
    fn exec(&self) -> Result<(), Error>;
}

#[derive(thiserror::Error, Debug, Clone)]
pub enum Error {
    #[error("Unknown Error: `{0}`")]
    Unknown(String),

    #[error("Internal Error: `{0}`")]
    Internal(String),

    #[error("Internal Git Error: `{0}`")]
    Git(String),

    #[error("Permission Denied")]
    PermissionDenied,

    #[error("Invalid Argument: `{0}`")]
    InvalidArgument(String),

    // TODO: Implement Other error using anyhow
}

impl Termination for Error {
    fn report(self) -> ExitCode {
        match self {
            Error::Unknown(_) => ExitCode::from(1),
            _ => ExitCode::from(255),
        }
    }
}
