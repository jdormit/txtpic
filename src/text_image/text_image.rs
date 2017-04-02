use std::fmt;
use std::ops::{Index,IndexMut};
use super::image::DynamicImage;
use ::character_set::CharacterSet;
use super::image_to_text::image_to_text;

pub struct TextImage {
    chars: Vec<Vec<char>>,
}

impl Index<usize> for TextImage {
    type Output = Vec<char>;

    fn index(&self, idx: usize) -> &Vec<char> {
        &self.chars[idx]
    }
}
 
impl IndexMut<usize> for TextImage {
    fn index_mut(&mut self, idx: usize) -> &mut Vec<char> {
        &mut self.chars[idx]
    }
}

impl fmt::Display for TextImage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for line in &self.chars {
            for &c in line {
                s.push(c);
            }
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}

impl TextImage {
    pub fn new(width: usize, height: usize) -> TextImage {
        let mut chars: Vec<Vec<char>> = Vec::new();
        for _ in 0..height {
            let line = vec![' '; width];
            chars.push(line);
        }
        TextImage { chars: chars } 
    }

    pub fn from(img: DynamicImage, char_set: CharacterSet, target_width: u32) -> TextImage {
        image_to_text(img, char_set, target_width)
    }

    pub fn set_char(&mut self, x: usize, y: usize, c: char) {
        self.chars[y][x] = c
    }
}
