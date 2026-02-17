use core::fmt;
use core::str::FromStr;

/// Domain-safe identifier for users.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UserId(u64);

/// Domain-safe identifier for orders.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OrderId(u64);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IdError {
    Zero,
    InvalidNumber { input: String },
}

impl fmt::Display for IdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Zero => write!(f, "id must be greater than zero"),
            Self::InvalidNumber { input } => write!(f, "invalid id value: {input}"),
        }
    }
}

impl std::error::Error for IdError {}

fn parse_non_zero(input: &str) -> Result<u64, IdError> {
    let n = input.parse::<u64>().map_err(|_| IdError::InvalidNumber {
        input: input.into(),
    })?;
    if n == 0 {
        return Err(IdError::Zero);
    }
    Ok(n)
}

impl UserId {
    pub fn new(raw: u64) -> Result<Self, IdError> {
        if raw == 0 {
            return Err(IdError::Zero);
        }
        Ok(Self(raw))
    }

    pub fn get(self) -> u64 {
        self.0
    }
}

impl OrderId {
    pub fn new(raw: u64) -> Result<Self, IdError> {
        if raw == 0 {
            return Err(IdError::Zero);
        }
        Ok(Self(raw))
    }

    pub fn get(self) -> u64 {
        self.0
    }
}

impl FromStr for UserId {
    type Err = IdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(parse_non_zero(s)?))
    }
}

impl FromStr for OrderId {
    type Err = IdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(parse_non_zero(s)?))
    }
}

/// A tiny example function showing type-safe boundaries.
///
/// ```compile_fail
/// use power_blocks::typed_id::{attach_order, OrderId, UserId};
///
/// let user = UserId::new(1).unwrap();
/// let order = OrderId::new(10).unwrap();
///
/// // Compile error: wrong argument order.
/// let _ = attach_order(order, user);
/// ```
pub fn attach_order(user_id: UserId, order_id: OrderId) -> String {
    format!("linked user={} order={}", user_id.get(), order_id.get())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_id_rejects_zero() {
        assert_eq!(UserId::new(0), Err(IdError::Zero));
    }

    #[test]
    fn order_id_from_str_valid() {
        let order: OrderId = "42".parse().unwrap();
        assert_eq!(order.get(), 42);
    }

    #[test]
    fn from_str_rejects_invalid_values() {
        assert!(matches!(
            "".parse::<UserId>(),
            Err(IdError::InvalidNumber { .. })
        ));
        assert!(matches!(
            "abc".parse::<OrderId>(),
            Err(IdError::InvalidNumber { .. })
        ));
        assert_eq!("0".parse::<UserId>(), Err(IdError::Zero));
        assert!(matches!(
            "18446744073709551616".parse::<OrderId>(),
            Err(IdError::InvalidNumber { .. })
        ));
    }

    #[test]
    fn attach_order_uses_typed_ids() {
        let user = UserId::new(7).unwrap();
        let order = OrderId::new(99).unwrap();
        assert_eq!(attach_order(user, order), "linked user=7 order=99");
    }
}
