use image::{open, GenericImageView, RgbaImage, Pixel};

fn main() {
    let penguin = open("linux-torvalds-penguin-painting.jpg").unwrap();
    let scalar = 50;

    let (width, height) = penguin.dimensions();
    let mut result = RgbaImage::new(width, height);

    for x in 0..width {
        for y in 0..height {
            let pixel = penguin.get_pixel(x, y);
            let channels = pixel.channels();
            let r = (channels[0] as i32 + scalar).max(0).min(255) as u8;
            let g = (channels[1] as i32 + scalar).max(0).min(255) as u8;
            let b = (channels[2] as i32 + scalar).max(0).min(255) as u8;
            let a = channels[3];
            result.put_pixel(x, y, image::Rgba([r, g, b, a]));
        }
    }

    result.save("result-addition-scalar.jpg").unwrap();
}