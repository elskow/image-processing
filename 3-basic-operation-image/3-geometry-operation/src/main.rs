use image::{open, GenericImageView, RgbaImage, Pixel, imageops};

fn image_rotating(img: &RgbaImage, angle: f32) -> RgbaImage {
    let (width, height) = img.dimensions();
    let mut new_img = RgbaImage::new(width, height);
    let center_x = width as f32 / 2.0;
    let center_y = height as f32 / 2.0;
    for x in 0..width {
        for y in 0..height {
            let x_f32 = x as f32;
            let y_f32 = y as f32;
            let x_diff = x_f32 - center_x;
            let y_diff = y_f32 - center_y;
            let new_x = (x_diff * angle.cos() - y_diff * angle.sin() + center_x) as u32;
            let new_y = (x_diff * angle.sin() + y_diff * angle.cos() + center_y) as u32;
            if new_x < width && new_y < height {
                new_img.put_pixel(new_x, new_y, *img.get_pixel(x, y));
            }
        }
    }
    new_img
}

fn image_translating(img: &RgbaImage, x: i32, y: i32) -> RgbaImage {
    let (width, height) = img.dimensions();
    let mut new_img = RgbaImage::new(width, height);
    for i in 0..width {
        for j in 0..height {
            let new_x = i as i32 + x;
            let new_y = j as i32 + y;
            if new_x >= 0 && new_x < width as i32 && new_y >= 0 && new_y < height as i32 {
                new_img.put_pixel(new_x as u32, new_y as u32, *img.get_pixel(i, j));
            }
        }
    }
    new_img
}

fn image_flip(img: &RgbaImage, horizontal: bool, vertical: bool) -> RgbaImage {
    let mut new_img = img.clone();
    match (horizontal, vertical) {
        (true, false) => {
            for x in 0..img.width() {
                for y in 0..img.height() {
                    let pixel = img.get_pixel(x, y);
                    new_img.put_pixel(img.width() - x - 1, y, *pixel);
                }
            }
        }
        (false, true) => {
            for x in 0..img.width() {
                for y in 0..img.height() {
                    let pixel = img.get_pixel(x, y);
                    new_img.put_pixel(x, img.height() - y - 1, *pixel);
                }
            }
        }
        (true, true) => {
            for x in 0..img.width() {
                for y in 0..img.height() {
                    let pixel = img.get_pixel(x, y);
                    new_img.put_pixel(img.width() - x - 1, img.height() - y - 1, *pixel);
                }
            }
        }
        (false, false) => {
            return new_img;
        }
    }

    new_img
}

fn image_zooming(img: &RgbaImage, factor: f32) -> RgbaImage {
    let (width, height) = img.dimensions();
    let new_width = (width as f32 * factor) as u32;
    let new_height = (height as f32 * factor) as u32;
    let mut new_img = RgbaImage::new(new_width, new_height);
    for x in 0..new_width {
        for y in 0..new_height {
            let old_x = (x as f32 / factor) as u32;
            let old_y = (y as f32 / factor) as u32;
            if old_x < width && old_y < height {
                new_img.put_pixel(x, y, *img.get_pixel(old_x, old_y));
            }
        }
    }
    new_img
}

fn main() {
    let img = open("stallman-meme.jpg").unwrap();
    let angle = 45.0_f32.to_radians();

    let rotated_img = image_rotating(&img.to_rgba8(), angle);
    rotated_img.save("stallman-meme-rotated.jpg").unwrap();


    let (x, y) = (100, 100);
    let translated_img = image_translating(&img.to_rgba8(), x, y);
    translated_img.save("stallman-meme-translated.jpg").unwrap();


    let flipped_img = image_flip(&img.to_rgba8(), true, false);
    flipped_img.save("stallman-meme-flipped-x.jpg").unwrap();
    let flipped_img = image_flip(&img.to_rgba8(), false, true);
    flipped_img.save("stallman-meme-flipped-y.jpg").unwrap();
    let flipped_img = image_flip(&img.to_rgba8(), true, true);
    flipped_img.save("stallman-meme-flipped-xy.jpg").unwrap();

    let factor = 5.0;
    let zoomed_img = image_zooming(&img.to_rgba8(), factor);
    println!("Before: {:?}", img.dimensions());
    println!("After: {:?}", zoomed_img.dimensions());
    zoomed_img.save("stallman-meme-zoomed.jpg").unwrap();
}
