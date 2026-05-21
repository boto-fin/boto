use crate::assets::AssetDecimals;

#[derive(Debug, thiserror::Error, Clone, PartialEq, Eq)]
pub enum AssetError {
    #[error("asset code cannot be empty")]
    EmptyAssetCode,

    #[error("asset decimals {actual} exceed maximum supported decimals {max}")]
    TooManyAssetDecimals { actual: u8, max: u8 },

    #[error("asset amounts use different assets: {left} and {right}")]
    AssetMismatch { left: String, right: String },

    #[error("asset amount must be greater than zero")]
    ZeroAmount,

    #[error("asset amount arithmetic overflowed")]
    ArithmeticOverflow,

    #[error("asset amount arithmetic underflowed")]
    ArithmeticUnderflow,
}

impl AssetError {
    pub(crate) fn too_many_decimals(actual: u8) -> Self {
        Self::TooManyAssetDecimals {
            actual,
            max: AssetDecimals::MAX,
        }
    }
}
