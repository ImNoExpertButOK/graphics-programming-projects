mod ppm;
use ppm::PPMImage;
mod vec3;
use vec3::Vec3;

fn main() {
    let o = PPMImage::new(600, 600, String::from("output"));
    o.save().expect("não foi possível criar o arquivo");

    let v1 = Vec3::new(1.0, 2.0, 3.0);
    let v2 = Vec3::new(4.0, 5.0, 6.0);

    let result = Vec3::dot(&v1, &v2);
    println!("Dot product: {}", result); // Output: 32.0
}
