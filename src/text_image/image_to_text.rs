use super::text_image::TextImage;
use super::image::DynamicImage;
use character_set::CharacterSet;

pub fn image_to_text(img: DynamicImage, char_set: CharacterSet, target_width_interval: u32, target_height_interval: u32) -> TextImage {
    let mut text_img = TextImage::new();
    let (img_width, img_height) = img.dimensions();
    let width_interval = closest_interval(target_width_interval, img_width);
    let height_interval = closest_interval(target_height_interval, img_height);

    // Iterate through each width_interval x height_interval chunk and calculate brightness
    for height_index in 0..img_height/height_interval {
        for width_index in 0..img_width/width_interval {
            for y in height_interval * height_index..height_interval * (height_index + 1) {
                for x in width_interval * width_index..width_interval * (width_index + 1) {
                    let [r, g, b] = img.get_pixel(x, y).to_rgb().data;
                    let brightness = (r + g + b) / 3;
                    let c = char_set.get(brightness as i32)
                }
            }
        }
    }
}

// Calculates the interval closest to target_interval such that img_size % target_interval == 0
fn closest_interval(target_interval: u32, img_size: u32) -> u32 {
    // Naive approach
    // TODO implement a timeout so this doesn't loop forever
    let mut upper_bound = target_interval;
    let mut lower_bound = target_interval;
    let mut res: u32;
    loop {
        if img_size % upper_bound == 0 {
            res = upper_bound;
            break;
        }
        else if img_size % lower_bound == 0 {
            res = lower_bound;
            break;
        }
        upper_bound++;
        lower_bound--;
    }
    res
}
