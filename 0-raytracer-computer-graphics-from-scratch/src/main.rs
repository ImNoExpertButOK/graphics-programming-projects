mod ppm;
use ppm::PPMImage;

fn main() {
    let o = PPMImage::new(600, 600, String::from("output"));
    o.save().expect("não foi possível criar o arquivo");
}
