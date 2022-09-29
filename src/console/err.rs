use std::io::Error;

pub enum IOError {
    UpstreamError(Error),
    MismatchedLength {required: usize, acquired: usize},
}

impl IOError {
    pub fn to_string(&self) -> String {
        match self {
            IOError::UpstreamError(e) => {
                "Upstream error: ".to_string()
                + &e.to_string()
            },
            IOError::MismatchedLength {required, acquired} => {
                "Mismatched Length: ".to_string()
                + "required "
                + &required.to_string()
                + ", but got "
                + &acquired.to_string()
                + "."
            },
        }
    }
}

impl From<Error> for IOError {
    fn from(e: Error) -> Self {
        Self::UpstreamError(e)
    }
}