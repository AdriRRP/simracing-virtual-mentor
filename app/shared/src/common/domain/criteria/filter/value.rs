use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Debug)] // TODO: Revisar clone
pub struct Value {
    filter_value: String,
}

impl Value {
    #[must_use]
    pub fn new(filter_value: &str) -> Self {
        Self {
            filter_value: filter_value.to_owned(),
        }
    }

    #[must_use]
    pub fn get(&self) -> &str {
        self.filter_value.as_ref()
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.filter_value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_to_string_ok() {
        let initial_value = "my value";
        let value = Value::new(initial_value);
        let current_value = value.to_string();

        assert_eq!(initial_value.to_owned(), current_value)
    }
}
