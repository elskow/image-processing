use image::{open, GenericImageView, RgbaImage, Pixel, imageops};

fn main() {
    let tree = open("lone-tree.jpg").unwrap();
    let mut moon = open("moon.jpg").unwrap();

    let (width, height) = tree.dimensions();
    moon = image::DynamicImage::ImageRgba8(imageops::resize(&moon, width, height, imageops::FilterType::Nearest));

    let mut result = RgbaImage::new(width, height);

    for x in 0..width {
        for y in 0..height {
            let tree_pixel = tree.get_pixel(x, y);
            let moon_pixel = moon.get_pixel(x, y);
            let tree_channels = tree_pixel.channels();
            let moon_channels = moon_pixel.channels();
            let (r1, g1, b1, a1) = (tree_channels[0],
                                    tree_channels[1],
                                    tree_channels[2],
                                    tree_channels[3]);
            let (r2, g2, b2, a2) = (moon_channels[0],
                                    moon_channels[1],
                                    moon_channels[2],
                                    moon_channels[3]);
            let r = r1.saturating_add(r2);
            let g = g1.saturating_add(g2);
            let b = b1.saturating_add(b2);
            let a = a1.saturating_add(a2);
            result.put_pixel(x, y, image::Rgba([r, g, b, a]));
        }
    }

    result.save("result-addition.png").unwrap();
}