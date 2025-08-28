use image::DynamicImage;
use image::Luma;
use std::{collections::HashMap, error::Error};

fn get_lines<'a>(
    (from, to): (u32, u32),
    hash: &'a HashMap<usize, &[u8]>,
    def_vec: &'a [u8],
) -> Vec<&'a [u8]> {
    let mut lines = Vec::new();
    for num in from..to {
        lines.push(*hash.get(&(num as usize)).unwrap_or(&def_vec));
    }
    lines
}
/// LBP (local binary pattern)
///
/// # Usage
///
/// ```
/// let args: Vec<String> = std::env::args().collect();
/// let path = Path::new(&args[1]);
/// let img = image::open(path)?;
/// let img = lbp_run(85, img)?;
/// img.save("mypic.png")?;
/// ```
///Bound is the bound between white and black of luma8. 255 - white, 0 - black.
/// If the bound is 75 and pixel `Some(Luma[90])`, LBP = 1, if pixel `Some(Luma[60])` LBP = 0
pub fn lbp_run(
    bound: usize,
    img: DynamicImage,
) -> std::result::Result<DynamicImage, Box<dyn Error>> {
    let window = (3, 3);
    let img = img.into_luma8();
    let (width, height) = img.dimensions();
    let image = Into::<DynamicImage>::into(img).into_bytes();
    let hash_pixs = image.chunks(width as usize).enumerate().collect();
    let mut imgbuf = vec![];
    let def_vec = vec![0_u8; width as usize];
    let def_vec = def_vec.as_slice();
    for line in 0..height {
        let [one_l, two_l, three_l, ..] = get_lines((line, line + 3), &hash_pixs, def_vec)[..]
        else {
            todo!()
        };

        let mut i_one = one_l.windows(3);
        let mut i_two = two_l.windows(3);
        let mut i_three = three_l.windows(3);
        let def_wind = vec![0_u8; window.0 as usize];
        let def_wind = def_wind.as_slice();
        for _pixels in 0..width {
            let w1 = i_one.next().unwrap_or(def_wind).iter();
            let w2 = i_two.next().unwrap_or(def_wind).iter();
            let w3 = i_three.next().unwrap_or(def_wind).iter();
            let mut all: Vec<_> = w1.chain(w2).chain(w3).collect();
            all.remove(5);
            let all: String = all
                .iter()
                .map(|x| {
                    if (**x as usize) < bound {
                        return '0';
                    }
                    '1'
                })
                .collect();
            let digit = u8::from_str_radix(&all, 2).expect("err");
            imgbuf.push(digit);
        }
    }
    use image::ImageBuffer;
    let img_buffer: ImageBuffer<Luma<u8>, Vec<u8>> =
        ImageBuffer::from_raw(width, height, imgbuf).unwrap();
    let img2 = DynamicImage::ImageLuma8(img_buffer);
    Ok(img2)
}
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
