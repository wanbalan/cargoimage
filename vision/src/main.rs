// use image::DynamicImage;
// use image::GrayImage;
// use image::ImageBuffer;
// use image::Luma;
use image::{self, buffer::ConvertBuffer};
use image::{GrayImage, Luma};
use scirs2_vision::feature::lbp_histogram;
use scirs2_vision::feature::template_matching::{draw_match, find_matches};
use std::path::Path;

fn calculate_lbp(image: &GrayImage) -> GrayImage {
    let (width, height) = image.dimensions();
    let mut lbp_image = GrayImage::new(width, height);

    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let center_pixel = image.get_pixel(x, y)[0];
            let mut lbp_value = 0_u8;

            // Define neighbors' relative coordinates
            let neighbors = [
                (-1, -1),
                (0, -1),
                (1, -1),
                (-1, 0),
                (1, 0),
                (-1, 1),
                (0, 1),
                (1, 1),
            ];

            for (i, (dx, dy)) in neighbors.iter().enumerate() {
                let neighbor_pixel =
                    image.get_pixel((x as i32 + dx) as u32, (y as i32 + dy) as u32)[0];
                if neighbor_pixel >= center_pixel {
                    lbp_value |= 1 << i;
                }
            }
            lbp_image.put_pixel(x, y, Luma([lbp_value]));
        }
    }
    lbp_image
}
fn main() {
    // let mut gi = image::GrayImage::new(100, 100);
    // let bi: ImageBuffer<Luma<u8>, Vec<_>> = gi.convert();
    let args: Vec<String> = std::env::args().collect();
    let path = Path::new(&args[1]);
    let img = image::open(path).expect("dsc").into_luma8();
    // img.save("maypic_luma8.png").unwrap();
    calculate_lbp(&img).save("lbp.png");
    // let hist = lbp_histogram(&img, 255, false).expect("err");
    // println!("{:#?}\nlen  {}", hist, hist.len());
}
