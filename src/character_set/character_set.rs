use std::collections::HashMap;
use std::i32;
use super::calculate_character_brightness::calculate_character_brightness;

#[derive(Debug)]
pub struct CharacterSet {
    // TODO brightnesses should be u32 not i32 to be more compatible with the image crate
    brightness_table: HashMap<i32, char>,
}

impl CharacterSet {
    // TODO generalize this so that chars is any iterable of chars
    pub fn new(chars: Vec<char>) -> CharacterSet {
        let mut set = CharacterSet { brightness_table: HashMap::new() };
        let mut init_brightness = HashMap::new();
        let mut min = i32::MAX;
        let mut max = 0;
        for c in chars {
            let brightness = calculate_character_brightness(c);
            min = if brightness < min { brightness } else { min };
            max = if brightness > max { brightness } else { max };
            init_brightness.insert(brightness, c); 
        }
        for (init_brightness, c) in init_brightness {
            let brightness = (255 * (init_brightness - min)) / (max - min);
            set.brightness_table.insert(brightness, c);
        }
        set
    }

    pub fn get(&self, brightness: i32) -> char {
        let mut upper = brightness;
        let mut lower = brightness;
        loop {
            match self.brightness_table.get(&upper) {
                Some(&c) => return c,
                None => upper += 1
            }
            match self.brightness_table.get(&lower) {
                Some(&c) => return c,
                None => lower -= 1
            }
        }
    }
}
