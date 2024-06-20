pub mod condition;
pub mod field;
pub mod value;

use condition::Condition;
use field::Field;
use value::Value;

#[derive(Eq, PartialEq, Clone, Debug)] // TODO: Revisar clone
pub struct Filter {
    field: Field,
    condition: Condition,
    value: Value,
}

impl Filter {
    #[must_use]
    pub const fn new(field: Field, operator: Condition, value: Value) -> Self {
        Self {
            field,
            condition: operator,
            value,
        }
    }

    #[must_use]
    pub fn field(&self) -> Field {
        self.field.clone() // Clone to keep original value ownership in struct
    }

    #[must_use]
    pub fn condition(&self) -> Condition {
        self.condition.clone() // Clone to keep original value ownership in struct
    }

    #[must_use]
    pub fn value(&self) -> Value {
        self.value.clone() // Clone to keep original value ownership in struct
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn filter_field_str() -> &'static str {
        "field_name"
    }
    fn filter_value_str() -> &'static str {
        "ohn"
    }

    fn test_filter() -> Filter {
        Filter::new(
            Field::new(filter_field_str()),
            Condition::Contains,
            Value::new(filter_value_str()),
        )
    }

    #[test]
    fn filter_field_ok() {
        let filter = test_filter();
        let filter_field = filter.field();
        let expected_filter_field = Field::new(filter_field_str());

        assert_eq!(filter_field, expected_filter_field)
    }

    #[test]
    fn filter_operator_ok() {
        let filter = test_filter();
        let filter_operator = filter.condition();
        let expected_filter_operator = Condition::Contains;

        assert_eq!(filter_operator, expected_filter_operator)
    }

    #[test]
    fn filter_value_ok() {
        let filter = test_filter();
        let filter_value = filter.value();
        let expected_filter_value = Value::new(filter_value_str());

        assert_eq!(filter_value, expected_filter_value)
    }
}
