use std::io::Error;

pub enum IOError {
    UpstreamError,
    MismatchedLength {required: usize, acquired: usize},
}

impl IOError {
    pub fn to_string(&self) -> String {
        match *self {
            IOError::UpstreamError => {
                "Error occurs during io manipulation!".to_string()
            },
            IOError::MismatchedLength {required, acquired} => {
                "Mismatched Length: ".to_string()
                + "required "
                + &required.to_string()
                + ", but got "
                + &acquired.to_string()
                + "."
            }
        }
    }
}

impl From<Error> for IOError {
    fn from(_: Error) -> Self {
        Self::UpstreamError
    }
}