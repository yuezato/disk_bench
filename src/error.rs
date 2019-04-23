use trackable;
use trackable::error::ErrorKindExt;

/// crate固有のエラー型.
#[derive(Debug, Clone, TrackableError)]
pub struct Error(trackable::error::TrackableError<ErrorKind>);
impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        if let Some(e) = e.get_ref().and_then(|e| e.downcast_ref::<Error>()).cloned() {
            e
        } else if e.kind() == std::io::ErrorKind::InvalidInput {
            ErrorKind::InvalidInput.cause(e).into()
        } else {
            ErrorKind::Other.cause(e).into()
        }
    }
}
impl From<Error> for std::io::Error {
    fn from(e: Error) -> Self {
        if *e.kind() == ErrorKind::InvalidInput {
            std::io::Error::new(std::io::ErrorKind::InvalidInput, e)
        } else {
            std::io::Error::new(std::io::ErrorKind::Other, e)
        }
    }
}

/// 発生し得るエラーの種別.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    InvalidInput,
    Other,
}
impl trackable::error::ErrorKind for ErrorKind {}
