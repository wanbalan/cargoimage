use image::ColorType;
use image::DynamicImage;
use std::{collections::HashMap, error::Error, path::Path};

fn my_test2(path: &Path) -> std::result::Result<(), Box<dyn Error>> {
    let window = (3, 3);
    let img = image::open(path).expect("err").into_luma8();
    let (width, height) = img.dimensions();
    let image = Into::<DynamicImage>::into(img).into_bytes();
    let hash_pixs = image.chunks(width as usize).enumerate().collect();
    fn get_lines<'a>((from, to): (u32, u32), hash: &'a HashMap<usize, &[u8]>) -> Vec<&'a &'a [u8]> {
        let mut lines = Vec::new();
        for num in from..to {
            lines.push(hash.get(&(num as usize)).expect("err"));
        }
        lines
    }
    let mut imgbuf = vec![];
    // let mut imgbuf = vec![0 as u8; ((width - 2) * (height - 2)) as usize];
    // let mut imgbuf: Vec<> = vec![vec![]; (width * height) as usize];

    for line in 0..height - 2 {
        let [one_l, two_l, three_l, ..] = get_lines((line, line + 3), &hash_pixs)[..] else {
            todo!()
        };
        let mut i_one = one_l.windows(3);
        let mut i_two = two_l.windows(3);
        let mut i_three = three_l.windows(3);
        for _pixels in 0..width - 2 {
            let w1 = i_one.next().expect("err").iter();
            let w2 = i_two.next().expect("err").iter();
            let w3 = i_three.next().expect("err").iter();
            let mut all: Vec<_> = w1.chain(w2).chain(w3).collect();
            all.remove(5);
            let all: String = all
                .iter()
                .map(|x| {
                    if (**x as usize) < 85 {
                        return '0';
                    }
                    '1'
                })
                .collect();
            let digit = u8::from_str_radix(&all, 2).expect("err");
            imgbuf.push(digit);
        }
        // if line == 5 {
        //     println!("{:#?}", imgbuf);
        //     panic!("sdc");
        // }
    }
    image::save_buffer_with_format(
        "mypic.png",
        &imgbuf,
        width - 2,
        height - 2,
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
