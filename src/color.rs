use image::Rgba;
use palette::{ Lab, Srgb, Hsl, IntoColor, FromColor, rgb::Rgb, color_difference::EuclideanDistance};
use std::convert::From;
use serde::Serialize;

#[derive(Clone, Copy, Serialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl Color {
    pub fn distance(&self, other: &Self) -> f64 {
        let self_lab: Lab = Lab::from_color(Srgb::new(self.r as f32 / 255.0, self.g as f32 / 255.0, self.b as f32 / 255.0));
        let other_lab: Lab = Lab::from_color(Srgb::new(other.r as f32 / 255.0, other.g as f32 / 255.0, other.b as f32 / 255.0));

        self_lab.distance_squared(other_lab).into()
    }
}

impl From<&Rgba<u8>> for Color {
    fn from(rgba: &Rgba<u8>) -> Self {
        Self {
            r: rgba[0],
            g: rgba[1],
            b: rgba[2]
        }
    }
}

impl From<Rgba<u8>> for Color {
    fn from(rgba: Rgba<u8>) -> Self {
        Self {
            r: rgba[0],
            g: rgba[1],
            b: rgba[2]
        }
    }
}

impl From<Hsl> for Color {
    fn from(hsl: Hsl) -> Self {
        let rgb: Srgb = hsl.into_color();
        let (r, g, b) = rgb.into_components();
        Self {
            r: (r * 255.0) as u8,
            g: (g * 255.0) as u8,
            b: (b * 255.0) as u8,
        }
    }
}

impl Into<Hsl> for Color {
    fn into(self) -> Hsl {
        Hsl::from_color(Srgb::new(self.r as f32 / 255.0, self.g as f32 * 255.0, self.b as f32 * 255.0))
    }
}

impl Into<Hsl> for &Color {
    fn into(self) -> Hsl {
        Hsl::from_color(Srgb::new(self.r as f32 / 255.0, self.g as f32 * 255.0, self.b as f32 * 255.0))
    }
}
