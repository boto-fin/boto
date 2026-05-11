use crate::assets::AssetError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AssetDecimals(u8);

impl AssetDecimals {
    pub const MAX: u8 = 38;

    pub fn new(value: u8) -> Result<Self, AssetError> {
        if value > Self::MAX {
            return Err(AssetError::too_many_decimals(value));
        }

        Ok(Self(value))
    }

    pub fn value(self) -> u8 {
        self.0
    }
}
