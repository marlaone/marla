#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RequiredString(String);

impl TryFrom<String> for RequiredString {
    type Error = InvalidRequiredString;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(InvalidRequiredString::Empty)
        } else {
            Ok(RequiredString(value))
        }
    }
}

impl TryFrom<&str> for RequiredString {
    type Error = InvalidRequiredString;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(InvalidRequiredString::Empty)
        } else {
            Ok(RequiredString(value.to_string()))
        }
    }
}

pub enum InvalidRequiredString {
    Empty,
}
