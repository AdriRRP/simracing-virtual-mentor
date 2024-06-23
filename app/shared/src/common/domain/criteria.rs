pub mod filter;
pub mod filters;
pub mod order;

use filters::Filters;
use order::Order;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Criteria {
    pub filters: Option<Filters>,
    pub order: Option<Order>,
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}

impl Criteria {
    #[must_use]
    pub const fn new(
        filters: Option<Filters>,
        order: Option<Order>,
        offset: Option<usize>,
        limit: Option<usize>,
    ) -> Self {
        Self {
            filters,
            order,
            offset,
            limit,
        }
    }

    #[must_use]
    pub fn has_filters(&self) -> bool {
        self.filters.clone().is_some_and(|f| !&f.is_empty()) // TODO: Review clone
    }

    #[must_use]
    pub fn has_order(&self) -> bool {
        self.order.clone().is_some_and(|o| !o.is_empty())
    }

    #[must_use]
    pub fn filters(&self) -> Option<Filters> {
        self.filters.clone() // Clone to keep original value ownership in struct
    }

    #[must_use]
    pub fn order(&self) -> Option<Order> {
        self.order.clone() // Clone to keep original value ownership in struct
    }

    #[must_use]
    pub const fn offset(&self) -> Option<usize> {
        self.offset
    }

    #[must_use]
    pub const fn limit(&self) -> Option<usize> {
        self.limit
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        !self.has_filters() && !self.has_order() && self.offset.is_none() && self.limit.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::filter::condition::Condition;
    use super::filter::field::Field;
    use super::filter::value::Value;
    use super::filter::Filter;
    use super::order::by::By;
    use super::order::r#type::Type;
    use super::*;

    fn test_order() -> Order {
        Order::new(By::new("field_name"), Type::parse_or_none("asc"))
    }

    fn test_filter() -> Filter {
        Filter::new(
            Field::new("field_name"),
            Condition::Contains,
            Value::new("ice"),
        )
    }

    fn test_filters() -> Filters {
        Filters::from(vec![test_filter()])
    }

    fn test_criteria() -> Criteria {
        Criteria::new(
            Some(test_filters()),
            Some(test_order()),
            Some(100),
            Some(100_000),
        )
    }

    #[test]
    fn criteria_has_filters_ok() {
        let criteria = test_criteria();
        assert!(criteria.has_filters())
    }

    #[test]
    fn criteria_has_order_ok() {
        let criteria = test_criteria();
        assert!(criteria.has_order())
    }

    #[test]
    fn criteria_filters_ok() {
        let criteria = test_criteria();
        let filters = criteria.filters();
        let expected_filters = Some(test_filters());
        assert_eq!(filters, expected_filters)
    }

    #[test]
    fn criteria_order_ok() {
        let criteria = test_criteria();
        let order = criteria.order();
        let expected_order = Some(test_order());
        assert_eq!(order, expected_order)
    }

    #[test]
    fn criteria_offset_ok() {
        let criteria = test_criteria();
        let offset = criteria.offset();
        let expected_offset = Some(100);
        assert_eq!(offset, expected_offset)
    }

    #[test]
    fn criteria_limit_ok() {
        let criteria = test_criteria();
        let limit = criteria.limit();
        let expected_limit = Some(100_000);
        assert_eq!(limit, expected_limit)
    }

    #[test]
    fn criteria_is_empty_ok() {
        let criteria = Criteria::default();
        assert!(criteria.is_empty())
    }

    #[test]
    fn criteria_is_empty_when_filters_are_empty_ok() {
        let mut criteria = Criteria::default();
        criteria.filters = Some(Filters::default());
        assert!(criteria.is_empty())
    }

    #[test]
    fn criteria_is_empty_when_order_is_empty_ok() {
        let mut criteria = Criteria::default();
        criteria.order = Some(Order::new(By::new(""), Type::None));
        assert!(criteria.is_empty())
    }
}
