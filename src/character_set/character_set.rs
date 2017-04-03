use std::collections::HashMap;
use std::i32;
use super::calculate_character_brightness::calculate_character_brightness;

#[derive(Debug)]
#[derive(PartialEq)]
/// A CharacterSet represents a set of characters mapped to brightness values
pub struct CharacterSet {
    brightness_table: HashMap<i32, char>,
}

impl CharacterSet {
    /// Generates a new CharacterSet containing `chars`
    pub fn new<T>(chars: T) -> CharacterSet where T: IntoIterator<Item=char> {
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

    /// Generates a new character set from the string `string`
    pub fn from(string: &str) -> CharacterSet {
        let chars: Vec<char> = string.chars().collect();
        CharacterSet::new(chars)
    }

    /// Returns the character with brightness closest to `brightness`
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

    /// Returns a predefined CharacterSet with few characters
    pub fn preset_small() -> CharacterSet {
        CharacterSet::from("@%#x+=:-. ")
    }

    /// Returns a predefined CharacterSet with a moderate number of characters
    pub fn preset_medium() -> CharacterSet {
        CharacterSet::from("$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`'. ")
    }

    /// Returns a predefined CharacterSet with many characters
    pub fn preset_large() -> CharacterSet {
        CharacterSet::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890!@#$%^&*()-_=+[{]}\\|;:'\",.<>/?")
    }
}
