extern crate txtpic;
extern crate image;
extern crate clap;
use clap::{App, Arg};
use std::path::Path;
use txtpic::character_set::CharacterSet;
use txtpic::text_image::TextImage;

/// The `txtpic` command
/// 
/// Run `txtpic --help` or check out [the readme](https://github.com/jdormit/txtpic) for usage.
fn main() {
    let matches = App::new("txtpic")
                        .version(env!("CARGO_PKG_VERSION"))
                        .author("Jeremy Dormitzer <jeremy.dormitzer@gmail.com>")
                        .about("Generates text representations of images")
                        .arg(Arg::with_name("IMAGE")
                             .help("The input image")
                             .required(true)
                             .index(1))
                        .arg(Arg::with_name("width")
                             .short("w")
                             .long("width")
                             .takes_value(true)
                             .value_name("WIDTH")
                             .help("An approximate width value for the result")
                             .validator(|arg: String| {
                                 match arg.parse::<u32>() {
                                     Err(_) => Err("Width must be a positive number".to_string()),
                                     Ok(_) => Ok(())
                                 }
                             }))
                        .arg(Arg::with_name("char_set")
                             .short("c")
                             .long("character-set")
                             .use_delimiter(false)
                             .takes_value(true)
                             .value_name("CHARACTERS")
                             .help("An alternate character set to use"))
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

    let width = match matches.value_of("width") {
        Some(width) => width.parse::<u32>().unwrap(),
        None => 80 as u32
    };

    let txt_img = TextImage::from(img, char_set, width);
    println!("{}", txt_img);
}