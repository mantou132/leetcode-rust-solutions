use std::ops::Try;
use std::fmt;

#[derive(Debug)]
pub enum Result<T, E: fmt::Debug> {
    Ok(T),
    Err(E),
}

impl<T,E: fmt::Debug> Result<T,E> {
    pub fn unwrap(self) -> T {
        match self {
            Result::Ok(v) => v,
            Result::Err(_e) => abort!("called `Result::unwrap()` on an `Err` value: {:?}", _e),
        }
    }
}

impl<T,E: fmt::Debug> Try for Result<T,E> {
    type Ok = T;
    type Error = E;

    fn into_result(self) -> ::std::result::Result<T,E> {
        match self {
            Result::Ok(v) => Ok(v),
            Result::Err(v) => Err(v),
        }
    }

    fn from_error(v: Self::Error) -> Self {
        Result::Err(v)
    }

    fn from_ok(v: Self::Ok) -> Self {
        Result::Ok(v)
    }
}
