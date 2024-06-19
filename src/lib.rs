use wasm_bindgen::prelude::*;
// use palette::{FromColor, Hsl, Lab, Srgb};
use std::collections::HashMap;
// use serde::Serialize;
use serde_wasm_bindgen::to_value;

use rand::{seq::SliceRandom, SeedableRng, rngs::StdRng, Rng };

pub mod color;
pub use color::Color;

fn k_means_pp(colors: &[Color], k: usize) -> Vec<Color> {
    let seed = [0u8; 32];
    let mut rng = StdRng::from_seed(seed);

    let mut centroids = Vec::with_capacity(k);

    centroids.push(*colors.choose(&mut rng).unwrap());

    while centroids.len() < k {
        let mut distances: Vec<f64> = colors.iter()
            .map(|color| centroids.iter().map(|centroid| color.distance(centroid))
            .min_by(|a,b| a.partial_cmp(b).unwrap()).unwrap()).collect();

        let sum: f64 = distances.iter().sum();
        for distance in &mut distances {
            *distance /= sum;
        }

        let cumulative_distances: Vec<f64> = distances.iter().scan(0.0, |acc, &x| {
            *acc += x;
            Some(*acc)
        }).collect();

        let r: f64 = rng.gen();
        let next_centroid_index = cumulative_distances.iter().position(|&x| x >= r).unwrap();
        centroids.push(colors[next_centroid_index]);
    }
    centroids
}

fn k_means(colors: &[Color], k: usize, iterations: usize) -> Vec<Color> {
    let mut centroids = k_means_pp(colors, k);

    for _ in 0..iterations {
        let mut clusters: HashMap<usize, Vec<Color>> = HashMap::new();

        for &color in colors {
            let centroid_idx = centroids
            .iter()
            .enumerate()
            .map(|(i, &centroid)|  (i, color.distance(&centroid)))
            .min_by(|a,b| a.1.partial_cmp(&b.1).unwrap())
            .unwrap()
            .0;
        clusters.entry(centroid_idx).or_default().push(color);

        }

        for (i, cluster) in clusters {
            if !cluster.is_empty() {
                let sum: (u32, u32, u32) = cluster.iter().fold((0, 0, 0), |acc, &color| {
                    (acc.0 + color.r as u32, acc.1 + color.g as u32, acc.2 + color.b as u32)
                });
                let len = cluster.len() as u32;
                centroids[i] = Color {
                    r: (sum.0 / len) as u8,
                    g: (sum.1 / len) as u8,
                    b: (sum.2 / len) as u8
                };
            }
        }
    }
    centroids
}

#[wasm_bindgen]
pub fn extract_palette(data: &[u8], n_colors: usize, precision: Option<usize>, n_size: Option<u32>) -> JsValue {
    let precision = precision.or(Some(12)).unwrap();
    let n_size = n_size.or(Some(100)).unwrap();

    let img = image::load_from_memory(data).expect("Failed to load image");

    let downsampled = img.resize(n_size, n_size, image::imageops::FilterType::Triangle);
    let colors: Vec<Color> = downsampled.to_rgba8().pixels().map(|p| Color::from(*p)).collect();
    
    let palette: Vec<Color> = k_means(&colors, n_colors, precision).iter().take(n_colors).map(|&c| c).collect();

    to_value(&palette).unwrap()
}

use palette::Hsl;
#[wasm_bindgen]
pub fn add_matching_tint(base_hue: f32, base_saturation: f32, base_lightness: f32, palette_data: &[u8]) -> Vec<u8> {
    let mut palette = Vec::new();
    for chunk in palette_data.chunks(3) {
        if let [r,g,b] = chunk {
            palette.push(Color {r: *r, g: *g, b: *b});
        }
    }

    let (mut hue_sum, mut saturation_sum, mut lightess_sum) = (0.0, 0.0, 0.0);
    let len = palette.len() as f32;

    for color in &palette {
        let hsl: Hsl = color.into();
        hue_sum += hsl.hue.into_degrees();
        saturation_sum += hsl.saturation;
        lightess_sum += hsl.lightness;
    }

    let average_hue = hue_sum / len;
    let average_saturation = saturation_sum / len;
    let average_lightness = lightess_sum / len;

    let matching_hue = (average_hue + base_hue) / 2.0;
    let matching_saturation = (average_saturation + base_saturation) / 2.0;
    let matching_lightness = (average_lightness + base_lightness) / 2.0;

    let new_color = Color::from(Hsl::new(matching_hue, matching_saturation, matching_lightness));
    vec![new_color.r, new_color.g, new_color.b]
}
