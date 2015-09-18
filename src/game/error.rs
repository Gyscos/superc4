use super::RuleError;

#[derive(Debug)]
pub enum Error {
    Rule(RuleError),
}

impl From<RuleError> for Error {
    fn from(err: RuleError) -> Self {
        Error::Rule(err)
    }
}
