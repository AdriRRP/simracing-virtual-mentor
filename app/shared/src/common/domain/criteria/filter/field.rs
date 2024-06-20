use std::fmt::Display;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Field {
    filter_field: String,
}

impl Field {
    #[must_use]
    pub fn new(filter_field: &str) -> Self {
        Self {
            filter_field: filter_field.to_owned(),
        }
    }

    #[must_use]
    pub fn name(&self) -> &str {
        self.filter_field.as_ref()
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.filter_field.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn field_value_ok() {
        let initial_field = "my_field";
        let field = Field::new(initial_field);
        let field_value = field.to_string();

        assert_eq!(initial_field.to_owned(), field_value)
    }
}
