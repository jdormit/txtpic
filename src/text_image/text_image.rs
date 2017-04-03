extern crate image;

use std::fmt;
use std::ops::{Index,IndexMut};
use self::image::DynamicImage;
use ::character_set::CharacterSet;
use super::image_to_text::image_to_text;

/// Represents an image constructed from Unicode text characters
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
    /// Returns a new TextImage filled with space characters with the given width and height
    pub fn new(width: usize, height: usize) -> TextImage {
        let mut chars: Vec<Vec<char>> = Vec::new();
        for _ in 0..height {
            let line = vec![' '; width];
            chars.push(line);
        }
        TextImage { chars: chars } 
    }

    /// Returns a new TextImage generated from the given image with the given character set
    ///
    /// `target_width` is a "goal" width - the actual image will end up with a width that 
    /// preserves the original image's aspect ratio
    ///
    /// # Example
    /// ```
    /// # extern crate image;
    /// # extern crate txtpic_lib;
    /// # fn main() {
    /// # use self::image::open;
    /// # use std::path::Path;
    /// # use txtpic_lib::character_set::CharacterSet;
    /// # use txtpic_lib::text_image::TextImage;
    /// // import necessary crates and modules...
    ///
    /// let img = open(Path::new("./example/cat.jpg")).unwrap();
    /// let char_set = CharacterSet::preset_small();
    /// let txt_img = TextImage::from(img, char_set, 80);
    /// let cute_kitty = "\
    /// ==xxxxxxx=======+++--++-+-.:--:--+:.-+---++++=xxxxxx=====xxxxxx==========++
    /// =xxxxxxx====xx=x=++=++++++.++-:--=+--++++++++=====xxx=++=xxxxxx============
    /// =xxxx=====xxxxxx+++++++=+--+x+:-=x+-+==+++=+===++=++++--+-+=xx====x========
    /// =====x+++xxxx=+++++=++++=+:-x+:-xx-:=x=+-+========+++++---+================
    /// +++++++--++++++=====++--=x=-=+-+=x--xxx+-+xxx======+++++---+===========++++
    /// --+-++---+++++======x=-:=xx-+=--+x-+xxx-:+x=+=======++++=+++=xx============
    /// +--+++---+++++x==+++=x+.-xx=-=--+=-+=xx=-=x=====xx=+++++==+===xxxxx========
    /// --++-++++++===xxx===+=x--xxx+++--+=++xxx+=+-+xxx-++--++++++++=xxxxxxxxxxxxx
    /// +++-+++=x=====x=++==++==+x=++++++++=+=xxxxx+-xxx++x=+=+++-+===xxxxxxxxxxxxx
    /// ++--+=++x====++=+xxx++xxx=xx+==++++==x%++xxxxxx+-x=++++=x=-++xxxxxxxxxxxxxx
    /// +++-++++++=xxxxx=+xxxxxx=:=x==+++++==xx+::-----+++--+===xxxx=xx%%%%%%%%%%%%
    /// +++--=+++xx===+=xx+++++---+=+=========+-++=++-----++==xxxxx+=xxxxxxxxxxxxxx
    /// ++---+===xxx==+++-++++===+-:-=xxx=xx==+:.--::---++==xx+--+++xxxxxxxxxxxxxxx
    /// +++++++=xxxxxx=+=++-------::+==xx====+++---++++++xxxxxx=x=xxxxxxx%%%%%%%%%%
    /// +----++++++++=xxxx=++++++-----+=====+--++-++=+==xxxxx=+=xxx%%%%%%%%%%%%xxxx
    /// +==+--=++++=====xxxxx===+++=--+===+=++--===+===xxxxx==xxxxxxxxx%%%%%%%%%%%%
    /// ++=+++++==++====xxxxxxx=+xxx=-==+++++=+=xx=x=xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
    /// -+++++++++========xxxxxx=x=xxxx=+x==-=xxxx===xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
    /// +-+++--+++====xx=xxxxxxxxxxxxxxxx=-+xxxxxxxxxxxxxxxxxx========xxxxxxxxxxxxx
    /// ++--++++++++=x=x====xxxxxxxxxxxxxx+=xxxxxxxxxxxxxxxxx=++++++-+=xxxxxxxxxxxx
    /// ===+---+=+++++++=xxxxxxxxxxxxxxxxx==xxxxxxxxxxxxxxxxx==+++--+=====+xxxxxxxx
    /// +++++-----+++++==++==xx=++++=xxxxxxxxx%xxxxxxxxxxxxx=+++++--+=x=x===xxxxxxx
    /// ++++++------++=+-+=+=++++-+-+-++=xxxxxxxxxxxxxxxxxx+++----+++==xxx==xxxxxxx
    /// ++==++++--+--+--++++++++++++++++========+==+=xxxxxx=++-::--+==xxxxxxxxxxxxx
    /// ";
    /// assert_eq!(txt_img.to_string(), cute_kitty);
    /// # }
    /// ```
    pub fn from(img: DynamicImage, char_set: CharacterSet, target_width: u32) -> TextImage {
        image_to_text(img, char_set, target_width)
    }

    /// Sets the character at (x, y) to c
    /// 
    /// # Example
    /// ```
    /// use txtpic_lib::text_image::TextImage;
    ///
    /// let mut txt_img = TextImage::new(5, 5);
    /// txt_img.set_char(1, 1, 'M');
    /// assert_eq!(txt_img[1][1], 'M');
    /// ```
    pub fn set_char(&mut self, x: usize, y: usize, c: char) {
        self.chars[y][x] = c
    }
}
