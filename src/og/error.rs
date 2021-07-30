use thiserror::Error;

use super::OpenGraphTag;

#[derive(Error, Debug)]
pub enum OpenGraphError {
    #[error("Unexpected Open Graph's tag value provided for tag \"{0}\". Received \"{1}\"")]
    UnexpectedValue(OpenGraphTag, String),
}
