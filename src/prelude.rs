//! This module simply brings the most common Scarlet functionality under a single namespace, to
//! prevent excessive imports. As of now, this prelude includes every trait in Scarlet, the
//! ubiquitous [`RGBColor`](color/struct.RGBColor.html), the associated parse error [`RGBParseError`](color/enum.RGBParseError.html), the important
//! [`Illuminant`](illuminants/enum.Illuminant.html), and nothing else. Of particular note is that any alternative color space found
//! in the [`colors`](colors/index.html) module is not included. Additionally, the Material color enums and structs are
//! not present.

pub use bound::Bound;
pub use color::{Color, RGBColor, RGBParseError};
pub use colorpoint::ColorPoint;
pub use illuminants::Illuminant;
