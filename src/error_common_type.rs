use std::fmt::Display;
use std::net::AddrParseError;
use std::{fs::File, net::Ipv6Addr};
use std::error::Error;

#[derive(Debug)]
enum AppError {
    Io(std::io::Error),
    Net(AddrParseError),
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for AppError { }

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::Io(e)
    }
}

impl From<AddrParseError> for AppError {
    fn from(e: AddrParseError) -> Self {
        AppError::Net(e)
    }
}


fn main() -> Result<(), AppError> {
    if cfg!(windows) {
        let _f = File::open("C:/Windows/win.ini").map_err(AppError::Io)?;
    } else {
        let _f = File::open("/dev/null").map_err(AppError::Io)?;
    }

    let _i = "::khk".parse::<Ipv6Addr>().map_err(AppError::Net)?;
    Ok(())
}