use image::{open, RgbaImage};

fn and_operation(img_1: RgbaImage, img_2: RgbaImage) -> RgbaImage {
    let (width, height) = img_1.dimensions();

    let mut img = RgbaImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let pixel_1 = img_1.get_pixel(x, y);
            let pixel_2 = img_2.get_pixel(x, y);

            let new_pixel = image::Rgba([
                pixel_1[0] & pixel_2[0],
                pixel_1[1] & pixel_2[1],
                pixel_1[2] & pixel_2[2],
                pixel_1[3] & pixel_2[3],
            ]);

            img.put_pixel(x, y, new_pixel);
        }
    }
    img
}

fn or_operation(img_1: RgbaImage, img_2: RgbaImage) -> RgbaImage {
    let (width, height) = img_1.dimensions();

    let mut img = RgbaImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let pixel_1 = img_1.get_pixel(x, y);
            let pixel_2 = img_2.get_pixel(x, y);

            let new_pixel = image::Rgba([
                pixel_1[0] | pixel_2[0],
                pixel_1[1] | pixel_2[1],
                pixel_1[2] | pixel_2[2],
                pixel_1[3] | pixel_2[3],
            ]);

            img.put_pixel(x, y, new_pixel);
        }
    }
    img
}


fn img_grayscale(img: RgbaImage) -> RgbaImage {
    let (width, height) = img.dimensions();
    let mut new_img = RgbaImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);

            let new_pixel = image::Rgba([
                (pixel[0] as f32 * 0.299) as u8,
                (pixel[1] as f32 * 0.587) as u8,
                (pixel[2] as f32 * 0.114) as u8,
                pixel[3],
            ]);

            new_img.put_pixel(x, y, new_pixel);
        }
    }
    new_img
}

fn not_operation(img: RgbaImage) -> RgbaImage {
    let (width, height) = img.dimensions();

    let mut new_img = RgbaImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);

            let new_pixel = image::Rgba([
                !pixel[0],
                !pixel[1],
                !pixel[2],
                pixel[3],
            ]);

            new_img.put_pixel(x, y, new_pixel);
        }
    }
    new_img
}

fn xor_operation(img_1: RgbaImage, img_2: RgbaImage) -> RgbaImage {
    let (width, height) = img_1.dimensions();

    let mut img = RgbaImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let pixel_1 = img_1.get_pixel(x, y);
            let pixel_2 = img_2.get_pixel(x, y);

            let new_pixel = image::Rgba([
                pixel_1[0] ^ pixel_2[0],
                pixel_1[1] ^ pixel_2[1],
                pixel_1[2] ^ pixel_2[2],
                !pixel_1[3] ^ pixel_2[3],
            ]);

            img.put_pixel(x, y, new_pixel);
        }
    }
    img
}

fn main() {
    let img_1 = open("circle.png").unwrap();
    let img_2 = open("star-1.jpg").unwrap();

    let width = std::cmp::min(img_1.width(), img_2.width());
    let height = std::cmp::min(img_1.height(), img_2.height());

    let img_1 = img_1.resize(width, height, image::imageops::FilterType::Nearest);
    let img_2 = img_2.resize(width, height, image::imageops::FilterType::Nearest);

    let img = xor_operation(img_1.to_rgba8(), img_2.to_rgba8());

    img.save("output-xor-operation.png").unwrap();
}