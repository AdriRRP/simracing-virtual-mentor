pub mod error;

use std::fmt;
use std::fmt::{Debug, Display};

use error::Error;

#[derive(Eq, PartialEq, Clone, Debug)] // TODO: Revisar clone
pub enum Type {
    Asc,
    Desc,
    None,
}

impl Type {
    // TODO: ¿Dejar enmascarado el error?
    #[must_use]
    pub fn parse_or_none(type_str: &str) -> Self {
        let result = Self::parse(type_str);
        result.unwrap_or(Self::None)
    }

    /// # Errors
    ///
    /// Will return `Err` if `type_str` is not a valid variant
    pub fn parse(type_str: &str) -> Result<Self, Error> {
        match &type_str.to_lowercase()[..] {
            "asc" => Ok(Self::Asc),
            "desc" => Ok(Self::Desc),
            "none" => Ok(Self::None),
            _ => Err(Error::NoSuchType(type_str.to_owned())),
        }
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        matches!(self, Self::None)
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match *self {
            Self::Asc => String::from("asc"),
            Self::Desc => String::from("desc"),
            Self::None => String::from("none"),
        };
        write!(f, "{str}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn type_asc_parse_ok() {
        let type_str = "asc";
        let r#type = Type::parse(type_str);

        assert_eq!(r#type, Ok(Type::Asc))
    }

    #[test]
    fn type_desc_parse_ok() {
        let type_str = "desc";
        let r#type = Type::parse(type_str);

        assert_eq!(r#type, Ok(Type::Desc))
    }

    #[test]
    fn type_none_parse_ok() {
        let type_str = "none";
        let r#type = Type::parse(type_str);

        assert_eq!(r#type, Ok(Type::None))
    }

    #[test]
    fn invalid_type_parse_ko() {
        let type_str = "escalabraguardas";
        let r#type = Type::parse(type_str);

        assert_eq!(r#type, Err(Error::NoSuchType(type_str.to_owned())))
    }

    #[test]
    fn invalid_type_parse_or_none_ok() {
        let type_str = "escalabraguardas";
        let r#type = Type::parse_or_none(type_str);

        assert_eq!(r#type, Type::None)
    }
}
