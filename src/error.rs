use failure::{*};

#[derive(Debug, Fail)]
pub enum AssError {
    #[fail(display = "Invalid Account file: {}", file)]
    InvalidAccountFile {
        err: String,
        file: String,
    },
    #[fail(display = "Could not find: {}", file)]
    NotFound {
        err: String,
        file: String
    },
    #[fail(display = "Permission denied to: {}", file)]
    PermissionDenied {
        err: String,
        file: String
    }
}
