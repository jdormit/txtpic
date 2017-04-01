extern crate txtpic;
extern crate image;
use std::path::Path;
use txtpic::character_set::CharacterSet;
use txtpic::text_image::TextImage;

fn main() {
    let chars = vec!['M', '@', '%', '#', 'x', '+', '=', ':', '-', '.', ' '];
    let char_set = CharacterSet::new(chars);
    let img = image::open(Path::new("/home/jeremy/Pictures/jeremy.jpg")).unwrap();
    let txt_img = TextImage::from(img, char_set);
    println!("{}", txt_img);
}
