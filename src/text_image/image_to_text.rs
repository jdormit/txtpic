use super::text_image::TextImage;
use super::image::{DynamicImage, GenericImage, Pixel};
use character_set::CharacterSet;

/// Converts an image to a TextImage using the given character set
/// 
/// `target_width` is a goal width - the actual width will be a value close to
/// `target_width` that preserves the image's aspect ratio
pub fn image_to_text(img: DynamicImage, char_set: CharacterSet, target_width: u32) -> TextImage {
    let (img_width, img_height) = img.dimensions();
    let width_interval = closest_interval(img_width / target_width, img_width);
    let height_interval = width_interval * 2;
    let width = img_width / width_interval;
    let height = img_height / height_interval;

    let mut text_img = TextImage::new(width as usize, height as usize);

    // Iterate through each width_interval x height_interval chunk and calculate brightness
    for height_index in 0..height {
        for width_index in 0..width {
            let mut total_brightness: u32 = 0;
            for y in height_interval * height_index..height_interval * (height_index + 1) {
                for x in width_interval * width_index..width_interval * (width_index + 1) {
                    let rgb  = img.get_pixel(x, y).to_rgb().data;
                    let (r, g, b) = (rgb[0] as u32, rgb[1] as u32, rgb[2] as u32);
                    total_brightness += (r + g + b) / 3;
                }
            }
            let area = width_interval * height_interval;
            let brightness = total_brightness as f32 / area as f32;
            let c = char_set.get(brightness as i32);
            text_img.set_char(width_index as usize, height_index as usize, c);
        }
    }
    text_img
}

// Calculates the interval closest to target_interval such that img_size % target_interval == 0
fn closest_interval(target_interval: u32, img_size: u32) -> u32 {
    // Naive approach
    let mut upper_bound = target_interval;
    let mut lower_bound = target_interval;
    let res: u32;
    loop {
        if img_size % upper_bound == 0 {
            res = upper_bound;
            break;
        }
        else if img_size % lower_bound == 0 {
            res = lower_bound;
            break;
        }
        upper_bound += 1;
        lower_bound -= 1;
    }
    res
}
