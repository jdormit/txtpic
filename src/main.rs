extern crate txtpic;
extern crate image;
extern crate clap;
use clap::{App, Arg};
use std::path::Path;
use txtpic::character_set::CharacterSet;
use txtpic::text_image::TextImage;

fn main() {
    let matches = App::new("txtpic")
                        .version("1.0")
                        .author("Jeremy Dormitzer <jeremy.dormitzer@gmail.com>")
                        .about("Generates text representations of images")
                        .arg(Arg::with_name("IMAGE")
                             .help("The input image")
                             .required(true)
                             .index(1))
                        .arg(Arg::with_name("char_set")
                             .short("c")
                             .long("character-set")
                             .takes_value(true)
                             .value_name("CHARACTERS")
                             .help("An alternate character set to use."))
                        .get_matches();
    
    let img_path = matches.value_of("IMAGE").unwrap();
    let img = match image::open(Path::new(img_path)) {
        Ok(img) => img,
        Err(e) => {
            println!("Unable to read image: {}", e);
            std::process::exit(1);
        }
    };
    
    let char_set = match matches.value_of("char_set") {
        Some(chars) => CharacterSet::from(&chars),
        None => CharacterSet::preset_small()
    };

    let txt_img = TextImage::from(img, char_set);
    println!("{}", txt_img);
}
