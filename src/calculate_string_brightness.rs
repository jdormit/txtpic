extern crate rusttype;
use self::rusttype::{FontCollection, Scale, point, PositionedGlyph};

/// Returns the brightness value between 0 and 255 for the input string.
///
/// Based on the algorithm described [here](http://mattmik.com/articles/ascii/ascii.html).
/// The string is rendered in the
/// [Inconsolata font](http://www.levien.com/type/myfonts/inconsolata.html) for the
/// purposes of determining the brightness.
///
/// # Examples
///
/// ```
/// calculate_string_brightness("c");
/// ```
///
pub fn calculate_string_brightness(string: &str) -> i32 {
    // Generate an in-memory bitmap image of the character
    // Code snippet taken from https://github.com/dylanede/rusttype/blob/master/examples/simple.rs
    let font_data = include_bytes!("Inconsolata-Regular.ttf");
    let collection = FontCollection::from_bytes(font_data as &[u8]);
    let font = collection.into_font().unwrap();

    let height: f32 = 12.4;

    let scale = Scale { x: height * 2.0, y: height };

    // The origin of a line of text is at the baseline (roughly where non-descending letters sit).
    // We don't want to clip the text, so we shift it down with an offset when laying it out.
    // v_metrics.ascent is the distance between the baseline and the highest edge of any glyph in
    // the font. That's enough to guarantee that there's no clipping.
    let v_metrics = font.v_metrics(scale);
    let offset = point(0.0, v_metrics.ascent);

    // Although this will typically be only one glyph, multiple glyphs are supported
    let glyphs: Vec<PositionedGlyph> = font.layout(string, scale, offset).collect();

    let mut num_pixels = 0.0;
    let mut total_brightness = 0.0;
    for g in glyphs {
        // The `v` parameter to the closure in g.draw() is a color value from 0 to 1
        g.draw(|_, _, v| {
            let positive_v = if v < 0.0 { 0.0 } else { v };
            let scaled_v = positive_v * 255.0;
            total_brightness += scaled_v;
            num_pixels += 1.0;
        })
    }
    if total_brightness == 0.0 || num_pixels == 0.0 {
        0
    }
    else {
        (total_brightness/num_pixels) as i32
    }
}
