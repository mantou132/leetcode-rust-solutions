use std::convert::From;
use std::error;


#[derive(Debug)]
pub struct Error;

impl<T: error::Error> From<T> for Error {
    fn from(_: T) -> Self {
        Self{}
    }
}


#[cfg(test)]
mod tests {
    use super::super::prelude::*;
    use std::fmt;
    use std::error::Error;

    #[derive(Debug)]
    struct MyError;

    impl fmt::Display for MyError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    impl Error for MyError {
        fn description(&self) -> &str {
            "MyError"
        }

        fn cause(&self) -> Option<&Error> {
            None
        }
    }

    #[test]
    fn error() {
        let _ : Result<(),super::Error> =
            (|| { Err(MyError)? })();
    }
}
