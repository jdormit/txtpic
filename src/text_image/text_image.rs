use std::vec;
use std::fmt;
use std::string;
use std::ops::{Index,IndexMut};
use super::image::DynamicImage;
use ::character_set::CharacterSet;
use super::image_to_text::image_to_text;

pub struct TextImage {
    chars: Vec<Vec<char>>,
}

impl Index<i32> for TextImage {
    type Output = Vec<char>;

    fn index(&self, idx: i32) -> Vec<char> {
        self.chars[idx]
    }
}
 
impl IndexMut<i32> for TextImage {
    fn index_mut(&mut self, idx: i32) -> mut Vec<char> {
        mut self.chars[idx]
    }
}

impl fmt::Display for TextImage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for line in self.chars {
            for c in line {
                s.push(c);
            }
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}

impl TextImage {
    fn new() -> TextImage {
        TextImage(chars: Vec::new())
    }

    pub fn from(img: DynamicImage, char_set: CharacterSet) -> TextImage {
        image_to_text(img, char_set)
    }

    // TODO write setter for characters
}
