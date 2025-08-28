use image::ColorType;
use std::{error::Error, path::Path};

fn my_test2(path: &Path) -> std::result::Result<(), Box<dyn Error>> {
    let img = image::open(path).expect("err").to_luma8();
    let mut pixels = img.pixels();
    let (width, height) = img.dimensions();
    let mut imgbuf: Vec<Vec<char>> = vec![vec![]; (width * height / 9) as usize];
    println!("{:#?}", imgbuf.len());
    let mut i = 0;
    let mut j = 0;
    for h in 0..height {
        i = j;
        if h != 0 && h % 3 == 0 {
            j += width / 3;
            i = j;
        }
        for w in 0..width {
            if w % 3 == 0 {
                pixels.by_ref().take(3).for_each(|x| {
                    let mut b = '0';
                    if x.0[0] > 70 {
                        b = '1';
                    }
                    imgbuf[i as usize].push(b);
                });
                i += 1;
            }
        }
    }
    imgbuf.iter_mut().for_each(|x| {
        x.remove(5);
    });
    let mut newimg: Vec<u8> = Vec::new();

    imgbuf.iter().for_each(|x| {
        let collect: String = x.iter().collect();
        let nu8 = u8::from_str_radix(collect.as_str(), 2).expect("err");
        newimg.push(nu8);
    });
    image::save_buffer_with_format(
        "mypic.png",
        &newimg,
        width / 3,
        height / 3,
        ColorType::L8,
        image::ImageFormat::Png,
    )
    .expect("err");
    Ok(())
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = Path::new(&args[1]);
    my_test2(path).expect("err");
}
