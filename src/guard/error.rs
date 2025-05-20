use thiserror::Error;

#[derive(Error, Debug)]
pub enum GuardError {
    #[error("Permission denied")]
    PermissionDenied,
}