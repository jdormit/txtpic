extern crate txtpic;
use txtpic::character_set::CharacterSet;

fn main() {
    let chars = vec!['M', '@', '%', '#', 'x', '+', '=', ':', '-', '.', ' '];
    let char_set = CharacterSet::new(chars);
    println!("{:?}", char_set);
}
