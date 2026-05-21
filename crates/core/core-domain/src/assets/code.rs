use crate::assets::AssetError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AssetCode(String);

impl AssetCode {
    pub fn new(value: impl Into<String>) -> Result<Self, AssetError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(AssetError::EmptyAssetCode);
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for AssetCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
