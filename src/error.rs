#[derive(Debug)]
pub struct Error {
    detail: Detail,
}

#[derive(Debug)]
enum Detail {
    IoError(std::io::Error),
    Error(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.detail {
            Detail::IoError(ref e) => e.fmt(fmt),
            Detail::Error(ref msg) => write!(fmt, "{}", msg),
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match self.detail {
            Detail::IoError(ref e) => e.description(),
            Detail::Error(ref msg) => &msg,
        }
    }

    fn cause(&self) -> Option<&std::error::Error> {
        match self.detail {
            Detail::IoError(ref e) => Some(e),
            Detail::Error(_) => None,
        }
    }
}

impl Error {
    pub fn new(msg: &str) -> Error {
        return Error {
            detail: Detail::Error(msg.to_owned()),
        };
    }

    pub fn from_io_error(e: std::io::Error) -> Error {
        return Error {
            detail: Detail::IoError(e),
        };
    }
}
