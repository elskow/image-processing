use image::{open, GenericImageView, RgbaImage, Pixel, imageops};

fn main() {
    let dot = open("dot.jpg").unwrap();
    let mut star = open("star-1.jpg").unwrap();

    let (width, height) = dot.dimensions();
    star = image::DynamicImage::ImageRgba8(imageops::resize(&star, width, height, imageops::FilterType::Nearest));

    let mut result = RgbaImage::new(width, height);

    for x in 0..width {
        for y in 0..height {
            let dot_pixel = dot.get_pixel(x, y);
            let star_pixel = star.get_pixel(x, y);
            let dot_channels = dot_pixel.channels();
            let star_channels = star_pixel.channels();
            let (r1, g1, b1, a1) = (dot_channels[0],
                                    dot_channels[1],
                                    dot_channels[2],
                                    dot_channels[3]);
            let (r2, g2, b2, a2) = (star_channels[0],
                                    star_channels[1],
                                    star_channels[2],
                                    star_channels[3]);
            let r = (r1 as i16 - r2 as i16).abs() as u8;
            let g = (g1 as i16 - g2 as i16).abs() as u8;
            let b = (b1 as i16 - b2 as i16).abs() as u8;
            let a = (a1 as i16 - a2 as i16).abs() as u8;
            result.put_pixel(x, y, image::Rgba([r, g, b, a]));
        }
    }

    result.save("result-subtraction.jpg").unwrap();
}