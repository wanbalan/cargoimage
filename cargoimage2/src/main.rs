mod lbp;
// use crate::lbp::tetatet::it_works;
use lbp::lbp_run;
use std::path::Path;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let path = Path::new(&args[1]);
    let img = image::open(path)?;
    let img = lbp_run(85, img)?;
    // let imgbuf = my_test2(path, 35, img).expect("err");
    img.save("mypic.png")?;
    Ok(())
}
