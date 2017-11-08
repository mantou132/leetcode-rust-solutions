use super::super::compat::prelude::*;
use super::super::os::OSError;
use super::super::ctype::isspace;
use super::{ignore, ScanError};
use super::delimit::DelimitedScanner;
use super::peek::Peekable;
use super::file::{FileSource, FileSink};

type Source = Peekable<FileSource>;

pub fn stdin() -> Result<DelimitedScanner<Source, fn(&mut Source) -> Result<(),ScanError<OSError>>>, OSError> {
    Ok(DelimitedScanner::new(
        Peekable::new(FileSource::new(0, 1024)?)?,
        |s| ignore(s, |c| isspace(*c))
    ))
}

pub fn stdout() -> Result<FileSink, OSError> {
    FileSink::new(1, 1024)
}
