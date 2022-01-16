use std::error::Error;
use std::fmt::Result as FmtResult;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum DatabaseError {}

impl Error for DatabaseError {}

impl Display for DatabaseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        writeln!(f, "error")
    }
}
