use super::filter::Filter;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Default, Debug)] // TODO: Review clone
pub struct Filters {
    filters: Vec<Filter>,
}

impl Filters {
    #[must_use]
    pub fn from(filters_vec: Vec<Filter>) -> Self {
        Self {
            filters: filters_vec,
        }
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.filters.is_empty()
    }
}

impl Deref for Filters {
    type Target = Vec<Filter>;

    /// Allows accessing the files in the collection via dereferencing.
    ///
    /// # Returns
    ///
    /// A reference to the vector of files.
    fn deref(&self) -> &Self::Target {
        &self.filters
    }
}

#[cfg(test)]
mod tests {
    use super::super::filter::condition::Condition;
    use super::super::filter::field::Field;
    use super::super::filter::value::Value;
    use super::*;

    fn filters() -> Vec<Filter> {
        vec![
            Filter::new(
                Field::new("rate"),
                Condition::GreaterThan,
                Value::new("0.666"),
            ),
            Filter::new(
                Field::new("phone"),
                Condition::Equal,
                Value::new("666844000"),
            ),
        ]
    }
    #[test]
    fn new_empty_filters_ok() {
        let events = Filters::default();
        assert!(events.is_empty())
    }

    #[test]
    fn new_events_ok() {
        let filters = Filters::from(filters());
        assert_eq!(filters.filters.len(), 2)
    }

    #[test]
    fn map_events_ok() {
        let filters = Filters::from(filters());

        let mapped_filters: Vec<String> = filters.iter().map(|e| e.field().to_string()).collect();

        assert_eq!(mapped_filters.len(), 2)
    }
}
