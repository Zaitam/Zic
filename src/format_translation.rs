use image::{open, ImageBuffer, Rgb, GenericImageView};
use crate::pixel::Pixel;

enum TestImages {
    AllWhite,
    TwoNearBlacks,
    Colors,
    Colors2,
    ChessBoard,
}

pub fn open_png(file_path: String) -> Vec<Pixel> {
    // Open the PNG image
    println!("{}", file_path);
    let img = image::open(file_path).unwrap();
    // Get the pixel data as a vec of (R, G, B) tuples
    img.pixels().map(|p| Pixel {
        r: p.2[0],
        g: p.2[1],
        b: p.2[2],
    }).collect()
}

#[cfg(test)]
mod tests {
    use std::fmt::format;
    use crate::format_translation::{open_png, TestImages};

    #[test]
    fn png_reader() {
        const image: TestImages = TestImages::Colors;
        let pixels = open_png(format!("images/{}.png", test_image(image)));
    }
    const test_image: fn(x: TestImages) -> &str = |x| match x {
        TestImages::AllWhite => "img1",
        TestImages::TwoNearBlacks => "img2",
        TestImages::Colors => "img3",
        TestImages::Colors2 => "img4",
        TestImages::ChessBoard => "img5",
    };
}