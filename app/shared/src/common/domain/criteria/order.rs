pub mod by;
pub mod r#type;

use by::By;
use r#type::Type;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)] // TODO: Review clone
pub struct Order {
    pub by: By,
    pub r#type: Type,
}

impl Order {
    #[must_use]
    pub const fn new(by: By, r#type: Type) -> Self {
        Self { by, r#type }
    }

    #[must_use]
    pub const fn new_desc(by: By) -> Self {
        Self::new(by, Type::Desc)
    }

    #[must_use]
    pub const fn by(&self) -> &By {
        &self.by
    }

    #[must_use]
    pub const fn r#type(&self) -> &Type {
        &self.r#type
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.by.field_name().is_empty() && matches!(self.r#type, Type::None)
    }

    pub fn apply<T>(&self, a: &T, b: &T) -> Ordering
    where
        T: Ord + ?Sized,
    {
        match self.r#type {
            Type::Asc if a > b => Ordering::Greater,
            Type::Asc if a < b => Ordering::Less,
            Type::Desc if a > b => Ordering::Less,
            Type::Desc if a < b => Ordering::Greater,
            _ => Ordering::Equal,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn order_by_ok() {
        let order = Order::new(By::new("field_name"), Type::parse_or_none("desc"));

        let order_by = order.by();
        let expected_order_by = By::new("field_name");

        assert_eq!(order_by, &expected_order_by)
    }

    #[test]
    fn order_by_type() {
        let order = Order::new(By::new("field_name"), Type::parse_or_none("desc"));

        let order_type = order.r#type();
        let expected_order_type = Type::Desc;

        assert_eq!(order_type, &expected_order_type)
    }
}
