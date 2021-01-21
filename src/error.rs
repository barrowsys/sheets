/*
 * --------------------
 * THIS FILE IS LICENSED UNDER MIT
 * THE FOLLOWING MESSAGE IS NOT A LICENSE
 *
 * <barrow@tilde.team> wrote this file.
 * by reading this text, you are reading "TRANS RIGHTS".
 * this file and the content within it is the gay agenda.
 * if we meet some day, and you think this stuff is worth it,
 * you can buy me a beer, tea, or something stronger.
 * -Ezra Barrow
 * --------------------
 */
#![allow(dead_code)]

use std::fmt;

#[doc(hidden)]
macro_rules! impl_from {
    ($from:path, $to:expr) => {
        impl From<$from> for ErrorKind {
            fn from(e: $from) -> Self {
                $to(e)
            }
        }
    };
}

pub type Result<T> = std::result::Result<T, ErrorKind>;

#[derive(Debug)]
pub enum ErrorKind {
    CTError(crossterm::ErrorKind),
    IoError(std::io::Error),
    SerdeError(serde_json::Error),
    CellRefParseError(String),
    CoordRefParseError(String),
    RelativeNumber(i16, u16),
    Custom(String),
}
impl ErrorKind {
    pub fn new(s: &str) -> Self {
        Self::Custom(String::from(s))
    }
    pub fn cell_ref(s: &str) -> Self {
        Self::CellRefParseError(String::from(s))
    }
    pub fn coord_ref(s: &str) -> Self {
        Self::CoordRefParseError(String::from(s))
    }
}
impl_from!(crossterm::ErrorKind, ErrorKind::CTError);
impl_from!(std::io::Error, ErrorKind::IoError);
impl_from!(serde_json::Error, ErrorKind::SerdeError);
impl std::error::Error for ErrorKind {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::IoError(e) => Some(e),
            //TODO: handle crossterm error variants
            Self::CTError(e) => Some(e),
            Self::SerdeError(e) => Some(e),
            _ => None,
        }
    }
}
impl fmt::Display for ErrorKind {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CTError(e) => write!(fmt, "Crossterm error occured: {}", e),
            Self::IoError(e) => write!(fmt, "IO-error occurred: {}", e),
            Self::SerdeError(e) => write!(fmt, "Serde error occured: {}", e),
            Self::CellRefParseError(s) => write!(fmt, "Error parsing CellRef {}", s),
            Self::CoordRefParseError(s) => write!(fmt, "Error parsing CoordRef {}", s),
            Self::RelativeNumber(r, a) => write!(fmt, "Relative number error: {}{:+}", a, r),
            Self::Custom(s) => write!(fmt, "Some error occurred: {}", s),
        }
    }
}

