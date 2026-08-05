mod ppm;
use ppm::PPMImage;

#[derive(Copy, Clone)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    fn dot(a: &Vec3, b: &Vec3) -> f64 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }
}

fn main() {
    let o = PPMImage::new(600, 600, String::from("output"));
    o.save().expect("não foi possível criar o arquivo");

    let v1 = Vec3::new(1.0, 2.0, 3.0);
    let v2 = Vec3::new(4.0, 5.0, 6.0);

    let result = Vec3::dot(&v1, &v2);
    println!("Dot product: {}", result); // Output: 32.0
}
