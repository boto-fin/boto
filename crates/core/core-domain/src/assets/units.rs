use crate::assets::AssetError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AtomicUnits(u128);

impl AtomicUnits {
    pub const ZERO: Self = Self(0);

    pub fn new(value: u128) -> Self {
        Self(value)
    }

    pub fn positive(value: u128) -> Result<Self, AssetError> {
        if value == 0 {
            return Err(AssetError::ZeroAmount);
        }

        Ok(Self(value))
    }

    pub fn value(self) -> u128 {
        self.0
    }

    pub fn checked_add(self, other: Self) -> Result<Self, AssetError> {
        self.0
            .checked_add(other.0)
            .map(Self)
            .ok_or(AssetError::ArithmeticOverflow)
    }

    pub fn checked_sub(self, other: Self) -> Result<Self, AssetError> {
        self.0
            .checked_sub(other.0)
            .map(Self)
            .ok_or(AssetError::ArithmeticUnderflow)
    }
}

impl From<u128> for AtomicUnits {
    fn from(value: u128) -> Self {
        Self::new(value)
    }
}
