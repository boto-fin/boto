use crate::assets::{AssetCode, AssetDecimals, AssetError};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Asset {
    code: AssetCode,
    decimals: AssetDecimals,
}

impl Asset {
    pub fn new(code: AssetCode, decimals: AssetDecimals) -> Self {
        Self { code, decimals }
    }

    pub fn stablecoin(code: impl Into<String>, decimals: u8) -> Result<Self, AssetError> {
        Ok(Self::new(
            AssetCode::new(code)?,
            AssetDecimals::new(decimals)?,
        ))
    }

    pub fn code(&self) -> &AssetCode {
        &self.code
    }

    pub fn decimals(&self) -> AssetDecimals {
        self.decimals
    }
}
