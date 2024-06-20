pub mod error;

use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::{Debug, Display};

use crate::common::domain::criteria::filter::condition::error::Error;

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Debug)] // TODO: Revisar clone
pub enum Condition {
    Equal,
    NotEqual,
    GreaterThan,
    LowerThan,
    Contains,
    NotContains,
}

impl Condition {
    /// # Errors
    ///
    /// Will return `Err` if `op_str` is not a valid variant
    pub fn parse(op_str: &str) -> Result<Self, Error> {
        match &op_str.to_lowercase()[..] {
            "=" => Ok(Self::Equal),
            "!=" => Ok(Self::NotEqual),
            ">" => Ok(Self::GreaterThan),
            "<" => Ok(Self::LowerThan),
            "contains" => Ok(Self::Contains),
            "not_contains" => Ok(Self::NotContains),
            invalid => Err(Error::NoSuchOperator(invalid.to_string())),
        }
    }

    pub fn apply<T>(&self, left: &T, right: &T) -> bool
    where
        T: Eq + Ord + ToString + ?Sized,
    {
        match self {
            Self::Equal => left == right,
            Self::NotEqual => left != right,
            Self::Contains => left.to_string().contains(&right.to_string()),
            Self::NotContains => !left.to_string().contains(&right.to_string()),
            Self::GreaterThan => left > right,
            Self::LowerThan => left < right,
        }
    }
}

impl Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match *self {
            Self::Equal => String::from("="),
            Self::NotEqual => String::from("!="),
            Self::GreaterThan => String::from(">"),
            Self::LowerThan => String::from("<"),
            Self::Contains => String::from("CONTAINS"),
            Self::NotContains => String::from("NOT_CONTAINS"),
        };
        write!(f, "{str}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operator_eq_parse_ok() {
        let op_str = "=";
        let op = Condition::parse(op_str);

        assert_eq!(op, Ok(Condition::Equal))
    }

    #[test]
    fn operator_ne_parse_ok() {
        let op_str = "!=";
        let op = Condition::parse(op_str);

        assert_eq!(op, Ok(Condition::NotEqual))
    }

    #[test]
    fn operator_gt_parse_ok() {
        let op_str = ">";
        let op = Condition::parse(op_str);

        assert_eq!(op, Ok(Condition::GreaterThan))
    }

    #[test]
    fn operator_lt_parse_ok() {
        let op_str = "<";
        let op = Condition::parse(op_str);

        assert_eq!(op, Ok(Condition::LowerThan))
    }

    #[test]
    fn operator_contains_parse_ok() {
        let op_str = "contains";
        let op = Condition::parse(op_str);

        assert_eq!(op, Ok(Condition::Contains))
    }

    #[test]
    fn operator_not_contains_parse_ok() {
        let op_str = "not_contains";
        let op = Condition::parse(op_str);

        assert_eq!(op, Ok(Condition::NotContains))
    }

    #[test]
    fn invalid_operator_parse_ko() {
        let op_str = "escalabraguardas";
        let op = Condition::parse(op_str);

        assert_eq!(op, Err(Error::NoSuchOperator(op_str.to_owned())))
    }
}
