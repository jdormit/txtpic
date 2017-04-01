use super::image::DynamicImage;
use character_set::CharacterSet;

pub fn image_to_text(img: DynamicImage, char_set: CharacterSet, target_width: u32) -> TextImage {
    let (img_width, img_height) = img.dimensions();
    let width = closest_text_width(target_width, img_width);
    let aspect_ratio = (img_width as f32) / (img_height as f32);
    let height = (width as f32 / aspect_ratio) as u32;
    let width_interval = img_width / width;
    let height_interval = img_height / height;

    // Iterate through each width_interval x height_interval chunk and calculate brightness
}

// Calculates the width closest to the target width such img_width % result == 0
fn closest_text_width(target_width: u32, img_width: u32) -> u32 {
    // Naive approach
    // TODO implement a timeout so this doesn't loop forever
    let mut upper_bound = target_width;
    let mut lower_bound = target_width;
    let mut res: u32;
    loop {
        if img_width % upper_bound == 0 {
            res = upper_bound;
            break;
        }
        else if img_width % lower_bound == 0 {
            res = lower_bound;
            break;
        }
        upper_bound++;
        lower_bound--;
    }
    res
}
