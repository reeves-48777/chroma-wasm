use wasm_bindgen::prelude::*;
use image::{GenericImageView, DynamicImage};
use palette::{FromColor, Hsl, Lab, Srgb};
use std::collections::HashMap;
use serde::Serialize;
use serde_wasm_bindgen::to_value;

#[wasm_bindgen]
pub fn get_dominant_color(data: &[u8]) -> JsValue {
    let img = image::load_from_memory(data).unwrap();
    let (width, height) = img.dimensions();

    let pixels = img.to_rgb8().pixels()
    .map(|p| (p.0[0], p.0[1], p.0[2])).collect::<Vec<_>>();

    let mut color_count: HashMap<(u8, u8, u8), u32> = HashMap::new();
    for color in pixels.iter() {
        *color_count.entry(*color).or_insert(0) += 1;
    }

    let mut sorted_colors: Vec<_> = color_count.iter().collect();
    sorted_colors.sort_by(|a,b| b.1.cmp(a.1));

    let top_colors: Vec<(u8, u8, u8)> = sorted_colors.iter().take(5).map(|(&color, _)| color).collect();

    to_value(&top_colors).unwrap()
}
