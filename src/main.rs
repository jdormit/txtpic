extern crate txtpic;
use txtpic::calculate_string_brightness::calculate_string_brightness;

fn main() {
    println!("{}", calculate_string_brightness("@"));
    println!("{}", calculate_string_brightness("%"));
    println!("{}", calculate_string_brightness("#"));
    println!("{}", calculate_string_brightness("x"));
    println!("{}", calculate_string_brightness("+"));
    println!("{}", calculate_string_brightness("="));
    println!("{}", calculate_string_brightness(":"));
    println!("{}", calculate_string_brightness("-"));
    println!("{}", calculate_string_brightness("."));
    println!("{}", calculate_string_brightness(" "));
}
