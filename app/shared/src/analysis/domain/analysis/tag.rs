use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Copy, Debug)]
pub enum Tag {
    Single(Base),
    Tendency(Base, Base),
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Default, Clone, Copy, Debug)]
pub enum Base {
    #[default]
    Stay,
    Increase(u8),
    Reduce(u8),
}

impl Tag {
    #[must_use]
    pub const fn stay() -> Self {
        Self::Single(Base::Stay)
    }

    #[must_use]
    pub const fn increase() -> Self {
        Self::Single(Base::Increase(0))
    }

    #[must_use]
    pub const fn reduce() -> Self {
        Self::Single(Base::Reduce(0))
    }

    pub fn increment(&mut self) {
        if let Self::Single(Base::Increase(value) | Base::Reduce(value)) = self {
            *value += 1;
        }
    }

    #[must_use]
    pub const fn increase_with_level(level: u8) -> Self {
        Self::Single(Base::Increase(level))
    }

    #[must_use]
    pub const fn reduce_with_level(level: u8) -> Self {
        Self::Single(Base::Reduce(level))
    }

    pub fn add_tendency(&mut self, other: &Self) {
        if let Self::Single(ref base1) = *self {
            if let Self::Single(ref base2) = *other {
                *self = Self::Tendency(*base1, *base2);
            }
        }
    }
}
