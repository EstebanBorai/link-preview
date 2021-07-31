use std::str::FromStr;
use std::string::ToString;

use crate::og::OpenGraphTag;
use crate::og::error::OpenGraphError;

pub enum Gender {
    Female,
    Male,
}

impl ToString for Gender {
    fn to_string(&self) -> String {
        match self {
            Gender::Female => String::from("female"),
            Gender::Male => String::from("male"),
        }
    }
}

impl FromStr for Gender {
    type Err = OpenGraphError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "female" => Ok(Gender::Female),
            "male" => Ok(Gender::Male),
            _ => Err(OpenGraphError::UnexpectedValue(
                OpenGraphTag::ProfileGender,
                s.to_string(),
            )),
        }
    }
}

/// Open Graph's Profile Object implementation
///
/// # References
/// - https://ogp.me/#type_profile
pub struct Profile {
    first_name: Option<String>,
    last_name: Option<String>,
    username: Option<String>,
    gender: Option<Gender>,
}
