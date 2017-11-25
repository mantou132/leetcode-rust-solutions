use std::ops::Try;
use std::fmt;
use super::option::Option;
use super::option::Option::Some;
use super::option::Option::None;


#[derive(Debug)]
pub enum Result<T, E: fmt::Debug> {
    Ok(T),
    Err(E),
}

impl<T,E: fmt::Debug> Result<T,E> {
    pub fn is_ok(&self) -> bool {
        match *self {
            Result::Ok(_) => true,
            Result::Err(_) => false,
        }
    }

    pub fn is_err(&self) -> bool {
        match *self {
            Result::Ok(_) => false,
            Result::Err(_) => true,
        }
    }

    pub fn ok(self) -> Option<T> {
        match self {
            Result::Ok(x) => Some(x),
            Result::Err(_) => None,
        }
    }

    pub fn err(self) -> Option<E> {
        match self {
            Result::Ok(_) => None,
            Result::Err(x) => Some(x),
        }
    }

    pub fn unwrap(self) -> T {
        match self {
            Result::Ok(v) => v,
            Result::Err(_e) => abort!("called `Result::unwrap()` on an `Err` value: {:?}", _e),
        }
    }

    pub fn and_then<U, F: FnOnce(T) -> Result<U, E>>(self, op: F) -> Result<U, E> {
        match self {
            Result::Ok(t) => op(t),
            Result::Err(e) => Result::Err(e),
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


#[cfg(test)]
mod tests {
    use super::Result;
    use super::Result::Err;

    #[derive(Debug)]
    struct MyError;

    fn error() -> Result<(), MyError> {
        Err(MyError)?
    }

    #[test]
    fn test_type() {
        error();
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic]
    fn test_unwrap() {
        error().unwrap();
    }
}
