use std::fs;

use color_thief::ColorFormat;
use image::{io::Reader as ImageReader, RgbImage};
use palette::{FromColor, Hsv, Srgb};
use rgb::RGB8;

use crate::build_utils::helpers::get_project_root;

const BOARDS_PATH: &str = "images/boards";
const NUM_COLORS: usize = 5;
const COLORS_FILENAME: &str = "complimentary-colors.txt";

pub fn write_all_complimentary_colors() -> anyhow::Result<()> {
    for dir in fs::read_dir(get_project_root().join(BOARDS_PATH))?.flatten() {
        let color_file = dir.path().join(COLORS_FILENAME);
        if color_file.exists() {
            continue;
        }
        let basename = dir.file_name().to_str().unwrap().to_string();
        let image = dir.path().join(format!("{basename}.png"));
        let img = ImageReader::open(image)?.decode()?;
        if let image::DynamicImage::ImageRgb8(img) = img {
            let dominant_colors = get_palette(&img, ColorFormat::Rgb);
            fs::write(
                color_file,
                get_complimentary_colors(dominant_colors)
                    .iter()
                    .map(|c| {
                        let hex = c.into_format::<u8>();
                        format!("#{:02x}{:02x}{:02x}", hex.red, hex.green, hex.blue)
                    })
                    .collect::<Vec<String>>()
                    .join("\n"),
            )?;
        } else {
            panic!("{img:?} is not an RGB image");
        }
    }
    Ok(())
}

fn get_palette(image: &RgbImage, color_format: ColorFormat) -> Vec<Hsv> {
    let vec = color_thief::get_palette(image, color_format, 10, 2).unwrap();
    vec.into_iter().map(get_hsv).collect()
}

fn get_hsv(color: RGB8) -> Hsv {
    Hsv::from_color(Srgb::new(color.r, color.g, color.b).into_format())
}

fn get_complimentary_colors(dominant_colors: Vec<Hsv>) -> Vec<Srgb> {
    // Usually boards have 2 dominant colors (one for light squares and one for dark squares).
    // Usually only one of these has a high saturation, so we go off of that one.
    let (saturation, hue) = if dominant_colors[0].saturation > dominant_colors[1].saturation {
        (dominant_colors[0].saturation, dominant_colors[0].hue)
    } else {
        (dominant_colors[1].saturation, dominant_colors[1].hue)
    };

    // Use colors with the same saturation as the dominant color,
    // hues that are evenly spaced around the color wheel but excluding the dominant color's hue,
    // and maximum brightness
    (1..=NUM_COLORS)
        .map(|i| {
            Hsv::new(
                hue + (i as f32 * 360.0 / (NUM_COLORS + 1) as f32),
                saturation,
                1.0,
            )
        })
        .map(Srgb::from_color)
        .collect()
}
