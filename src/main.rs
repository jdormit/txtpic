extern crate txtpic_lib;
extern crate image;
extern crate clap;
use clap::{App, Arg};
use std::path::Path;
use txtpic_lib::character_set::CharacterSet;
use txtpic_lib::text_image::TextImage;

fn main() {
    let additional_help = "\
Note: the --width option attempts find a width close to the target width that preserves the \
aspect ratio of the original image. For certain images, there may be only one or two valid \
widths within a reasonable range, so the --width option may not appear to have an effect. In this \
case, try extremely high or extremely low width values to affect the output.
";
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
                             .default_value("80")
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
                        .arg(Arg::with_name("invert")
                             .short("i")
                             .long("invert")
                             .help("Invert the result to make it suitable for black text on a white background"))
                        .arg(Arg::with_name("preset")
                             .short("p")
                             .long("preset")
                             .value_name("PRESET NAME")
                             .conflicts_with("char_set")
                             .possible_values(&["small", "medium", "large"])
                             .default_value("small")
                             .help("A preset character set to use"))
                        .after_help(additional_help)
                        .get_matches();
    
    let img_path = matches.value_of("IMAGE").unwrap();
    let img = match image::open(Path::new(img_path)) {
        Ok(img) => img,
        Err(e) => {
            println!("Unable to read image: {}", e);
            std::process::exit(1);
        }
    };

    let mut char_set = match matches.value_of("char_set") {
        Some(chars) => CharacterSet::from(&chars),
        None => match matches.value_of("preset").unwrap() {
            "small" => CharacterSet::preset_small(),
            "medium" => CharacterSet::preset_medium(),
            "large" => CharacterSet::preset_large(),
            &_ => panic!("Unknown preset: {}", matches.value_of("preset").unwrap())
        }
    };

    if matches.is_present("invert") { char_set = char_set.invert(); };

    let width = matches.value_of("width").unwrap().parse::<u32>().unwrap();

    let txt_img = TextImage::from(img, char_set, width);
    print!("{}", txt_img);
}
