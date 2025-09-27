#[derive(Debug)]
pub enum FlowParseError {
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
    Serde(serde_json::Error),
    Invalid(String),
}

impl std::fmt::Display for FlowParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FlowParseError::Io(e) => write!(f, "IO error: {}", e),
            FlowParseError::Parse(e) => write!(f, "Parse error: {}", e),
            FlowParseError::Serde(e) => write!(f, "Serialization error: {}", e),
            FlowParseError::Invalid(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for FlowParseError {}

impl From<std::io::Error> for FlowParseError {
    fn from(err: std::io::Error) -> FlowParseError {
        FlowParseError::Io(err)
    }
}

impl From<std::num::ParseIntError> for FlowParseError {
    fn from(err: std::num::ParseIntError) -> FlowParseError {
        FlowParseError::Parse(err)
    }
}

impl From<serde_json::Error> for FlowParseError {
    fn from(err: serde_json::Error) -> FlowParseError {
        FlowParseError::Serde(err)
    }
}

impl From<String> for FlowParseError {
    fn from(message: String) -> FlowParseError {
        FlowParseError::Invalid(message)
    }
}
