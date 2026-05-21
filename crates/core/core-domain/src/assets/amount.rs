use std::cmp::Ordering;

use crate::assets::{Asset, AssetError, AtomicUnits};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AssetAmount {
    units: AtomicUnits,
    asset: Asset,
}

impl AssetAmount {
    pub fn from_atomic_units(units: AtomicUnits, asset: Asset) -> Self {
        Self { units, asset }
    }

    pub fn positive(units: AtomicUnits, asset: Asset) -> Result<Self, AssetError> {
        if units == AtomicUnits::ZERO {
            return Err(AssetError::ZeroAmount);
        }

        Ok(Self { units, asset })
    }

    pub fn units(&self) -> AtomicUnits {
        self.units
    }

    pub fn asset(&self) -> &Asset {
        &self.asset
    }

    pub fn checked_add_same_asset(&self, other: &Self) -> Result<Self, AssetError> {
        self.ensure_same_asset(other)?;

        Ok(Self {
            units: self.units.checked_add(other.units)?,
            asset: self.asset.clone(),
        })
    }

    pub fn checked_sub_same_asset(&self, other: &Self) -> Result<Self, AssetError> {
        self.ensure_same_asset(other)?;

        Ok(Self {
            units: self.units.checked_sub(other.units)?,
            asset: self.asset.clone(),
        })
    }

    pub fn cmp_same_asset(&self, other: &Self) -> Result<Ordering, AssetError> {
        self.ensure_same_asset(other)?;

        Ok(self.units.cmp(&other.units))
    }

    pub fn is_at_least(&self, other: &Self) -> Result<bool, AssetError> {
        Ok(matches!(
            self.cmp_same_asset(other)?,
            Ordering::Equal | Ordering::Greater
        ))
    }

    fn ensure_same_asset(&self, other: &Self) -> Result<(), AssetError> {
        if self.asset != other.asset {
            return Err(AssetError::AssetMismatch {
                left: self.asset.code().to_string(),
                right: other.asset.code().to_string(),
            });
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assets::{Asset, AtomicUnits};

    fn usdc() -> Asset {
        Asset::stablecoin("USDC", 6).unwrap()
    }

    fn usdt() -> Asset {
        Asset::stablecoin("USDT", 6).unwrap()
    }

    #[test]
    fn positive_rejects_zero_units() {
        let result = AssetAmount::positive(AtomicUnits::ZERO, usdc());

        assert_eq!(result, Err(AssetError::ZeroAmount));
    }

    #[test]
    fn checked_add_same_asset_adds_atomic_units() {
        let left = AssetAmount::positive(AtomicUnits::new(100), usdc()).unwrap();
        let right = AssetAmount::positive(AtomicUnits::new(25), usdc()).unwrap();

        let result = left.checked_add_same_asset(&right).unwrap();

        assert_eq!(result.units(), AtomicUnits::new(125));
    }

    #[test]
    fn checked_add_same_asset_rejects_asset_mismatch() {
        let left = AssetAmount::positive(AtomicUnits::new(100), usdc()).unwrap();
        let right = AssetAmount::positive(AtomicUnits::new(25), usdt()).unwrap();

        let result = left.checked_add_same_asset(&right);

        assert!(matches!(result, Err(AssetError::AssetMismatch { .. })));
    }

    #[test]
    fn is_at_least_compares_same_asset_amounts() {
        let requested = AssetAmount::positive(AtomicUnits::new(100), usdc()).unwrap();
        let settled = AssetAmount::positive(AtomicUnits::new(125), usdc()).unwrap();

        let result = settled.is_at_least(&requested).unwrap();

        assert!(result);
    }
}
