pub type Error<T> = Result<T, Box<dyn std::error::Error>>;


#[derive(Debug, Clone)]
pub enum RoError {
	Failure(&'static str)
}

impl std::error::Error for RoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl std::fmt::Display for RoError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match *self {
			RoError::Failure(msg) => write!(f, "{}", msg),
		}
    }
}