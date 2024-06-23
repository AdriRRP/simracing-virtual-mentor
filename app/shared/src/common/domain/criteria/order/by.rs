use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Debug)] // TODO: Revisar clone
pub struct By {
    pub field_name: String,
}

impl By {
    #[must_use]
    pub fn new(field_name: &str) -> Self {
        Self {
            field_name: field_name.to_owned(),
        }
    }

    #[must_use]
    pub const fn field_name(&self) -> &String {
        &self.field_name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn field_value_ok() {
        let initial_field_name = "my_field";
        let by = By::new(initial_field_name);
        let field_name = by.field_name();

        assert_eq!(initial_field_name, field_name)
    }
}
