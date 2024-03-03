use image::{open, GenericImageView, RgbaImage, Pixel};

fn main() {
    let penguin = open("linux-torvalds-penguin-painting.jpg").unwrap();
    let mask = [[1,1,1], [1,1,1], [1,1,1]];

    let (width, height) = penguin.dimensions();
    let mut result = RgbaImage::new(width, height);

    for x in 1..width-1 {
        for y in 1..height-1 {
            let mut r: i32 = 0;
            let mut g: i32 = 0;
            let mut b: i32 = 0;
            let mut a: i32 = 0;
            for i in 0usize..3 {
                for j in 0usize..3 {
                    let pixel = penguin.get_pixel(x+(i as u32)-1,
                                                  y+(j as u32)-1);
                    let channels = pixel.channels();
                    r = r.saturating_add((channels[0] * mask[i][j]).into());
                    g = g.saturating_add((channels[1] * mask[i][j]).into());
                    b = b.saturating_add((channels[2] * mask[i][j]).into());
                    a = a.saturating_add((channels[3] * mask[i][j]).into());
                }
            }
            result.put_pixel(x,
                             y,
                             image::Rgba(
                                 [(r/9).try_into().unwrap(),
                                     (g/9).try_into().unwrap(),
                                     (b/9).try_into().unwrap(),
                                     (a/9).try_into().unwrap()]));        }
    }

    result.save("result-multiplication.jpg").unwrap();
}