use super::*;

/// Represents where the mask is placed.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MaskPositionPoint {
    /// Placed on forehead.
    Forehead,
    /// Placed on eyes.
    Eyes,
    /// Placed on mouth.
    Mouth,
    /// Placed on chin.
    Chin,
}

/// Represents a [`MaskPosition`].
///
/// [`MaskPosition`]: https://core.telegram.org/bots/api#maskposition
#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct MaskPosition {
    /// The position point of the mask.
    pub point: MaskPositionPoint,
    /// The shift of the mask by X.
    pub x_shift: f64,
    /// The shift of the mask by Y.
    pub y_shift: f64,
    /// The scale of the mask.
    pub scale: f64,
}
