/// This file defines the Color trait and all of the standard color types that implement it.

use std::convert::From;
use std::string::ToString;
extern crate termion;
use self::termion::color::{Fg, Reset, Rgb};



/// A point in the CIE 1931 XYZ color space.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct XYZColor {
    // these need to all be positive
    // TODO: way of implementing this constraint in code?
    x: f64,
    y: f64,
    z: f64,
    // TODO: deal with illuminant
}

impl From<Vec<f64>> for XYZColor {
    fn from(nums: Vec<f64>) -> Self {
        XYZColor{x: nums[0], y: nums[1], z: nums[2]}
    }
}

impl Into<Vec<f64>> for XYZColor {
    fn into(self) -> Vec<f64> {
        vec![self.x, self.y, self.z]
    }
}


/// A trait that includes any color representation that can be converted to and from the CIE 1931 XYZ
/// color space.
pub trait Color {
    fn from_xyz(XYZColor) -> Self;
    fn into_xyz(&self) -> XYZColor;

    fn convert<T: Color>(&self) -> T {
        T::from_xyz(self.into_xyz())
    }
    fn write_colored_str(&self, text: &str) -> String {
        let rgb: RGBColor = self.convert();
        rgb.base_write_colored_str(text)
    }
}

impl Color for XYZColor {
    fn from_xyz(xyz: XYZColor) -> XYZColor {
        xyz
    }
    fn into_xyz(&self) -> XYZColor {
        *self
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct RGBColor {
    r: u8,
    g: u8,
    b: u8,
    // TODO: add exact unclamped versions of each of these
}
    
impl RGBColor {
    /// Given a string, returns that string wrapped in codes that will color the foreground. Used for
    /// the trait implementation of write_colored_str, which should be used instead.
    fn base_write_colored_str(&self, text: &str) -> String {
        format!("{code}{text}{reset}",
                code=Fg(Rgb(self.r, self.g, self.b)),
                text=text,
                reset=Fg(Reset)
        )
    }
}

impl Into<Vec<u8>> for RGBColor {
    fn into(self) -> Vec<u8> {
        vec![self.r, self.g, self.b]
    }
}

impl From<Vec<u8>> for RGBColor {
    fn from(nums: Vec<u8>) -> Self {
        RGBColor {r: nums[0], g: nums[1], b: nums[2]}
    }
}

impl ToString for RGBColor {
    fn to_string(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}

impl Color for RGBColor {
    fn from_xyz(xyz: XYZColor) -> RGBColor {
        // TODO: implement full illuminant list from
        // https://github.com/hughsie/colord/tree/master/data/illuminant
        // and deal with observers

        // first, get linear RGB values (i.e., without gamma correction)
        // https://en.wikipedia.org/wiki/SRGB#Specification_of_the_transformation

        // note how the diagonals are large: X, Y, Z, roughly equivalent to R, G, B
        let rgb_lin_vec = vec![3.2406 * xyz.x - 1.5372 * xyz.y - 0.4986 * xyz.z,
                               -0.9689 * xyz.x + 1.8758 * xyz.y + 0.0415 * xyz.z,
                               0.0557 * xyz.x - 0.2040 * xyz.y + 1.0570 * xyz.z];
        // now we scale for gamma correction
        let gamma_correct = |x: &f64| {
            if x <= &0.0031308 {
                &12.92 * x
            }
            else {
                &1.055 * x.powf(&1.0 / &2.4) - &0.055
            }
        };
        let float_vec:Vec<f64> = rgb_lin_vec.iter().map(gamma_correct).collect();
        // now rescale between 0 and 255 and cast to integers
        // TODO: deal with clamping and exact values
        // we're going to clamp values to between 0 and 255
        let clamp = |x: &f64| {
            if *x >= 1.0 {
                1.0
            } else if *x <= 0.0 {
                0.0
            } else {
                *x
            }
        };
        let rgb:Vec<u8> = float_vec.iter().map(clamp).map(|x| (x * 255.0).round() as u8).collect();
        
        RGBColor {
            r: rgb[0],
            g: rgb[1],
            b: rgb[2]
        }
    }

    fn into_xyz(&self) -> XYZColor {
        // scale from 0 to 1 instead
        // TODO: use exact values here?
        let uncorrect_gamma = |x: &f64| {
            if x <= &0.04045 {
                x / &12.92
            }
            else {
                ((x + &0.055) / &1.055).powf(2.4)
            }
        };
        let scaled_vec: Vec<f64> = vec![self.r, self.g, self.b].iter().map(|x| (*x as f64) / 255.0).collect();
        let rgb_vec: Vec<f64> = scaled_vec.iter().map(uncorrect_gamma).collect();

        // essentially the inverse of the above matrix multiplication
        let x = 0.4124 * rgb_vec[0] + 0.3576 * rgb_vec[1] + 0.1805 * rgb_vec[2];
        let y = 0.2126 * rgb_vec[0] + 0.7152 * rgb_vec[1] + 0.0722 * rgb_vec[2];
        let z = 0.0193 * rgb_vec[0] + 0.1192 * rgb_vec[1] + 0.9505 * rgb_vec[2];

        XYZColor{x, y, z}
    }
}
            
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn can_display_colors() {
        let b = 128;
        for i in 0..8 {
            let mut line = String::from("");
            let r = i * 16;
            for j in 0..8 {
                let g = j * 16;
                line.push_str(RGBColor{r, g, b}.write_colored_str("■").as_str());                
            }
            println!("{}", line);        }
    }
    
    #[test]
    fn xyz_to_rgb() {
        let xyz = XYZColor{x: 0.41874, y: 0.21967, z: 0.05649};
        let rgb: RGBColor = xyz.convert();
        assert_eq!(rgb.r, 254);
        assert_eq!(rgb.g, 23);
        assert_eq!(rgb.b, 55);
    }

    #[test]
    fn rgb_to_xyz() {
        let rgb = RGBColor{r: 45, g: 28, b: 156};
        let xyz: XYZColor = rgb.into_xyz();
        // these won't match exactly cuz floats, so I just check within a margin
        assert!((xyz.x - 0.0750).abs() <= 0.01);
        assert!((xyz.y - 0.0379).abs() <= 0.01);
        assert!((xyz.z-  0.3178).abs() <= 0.01);
    }
    #[test]
    fn test_xyz_color_display() {
        println!();
        let y = 0.5;
        for i in 0..21 {
            let mut line = String::from("");
            for j in 0..21 {
                let x = i as f64 * 0.94 / 20.0;
                let z = j as f64 * 1.08883 / 20.0;
                line.push_str(XYZColor{x, y, z}.write_colored_str("■").as_str());
            }

            println!("{}", line);
        }
    }
    #[test]
    fn test_rgb_to_string() {
        let c1 = RGBColor::from(vec![0, 0, 0]);
        let c2 = RGBColor::from(vec![244, 182, 33]);
        let c3 = RGBColor::from(vec![0, 255, 0]);
        assert_eq!(c1.to_string(), "#000000");
        assert_eq!(c2.to_string(), "#F4B621");
        assert_eq!(c3.to_string(), "#00FF00");
    }
}
