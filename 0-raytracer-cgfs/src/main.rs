mod ppm;
use ppm::PPMImage;
mod vec3;
use vec3::Vec3;
mod shapes;
use shapes::Sphere;

fn main() {
    let ball_a = Sphere {
        center: Vec3::new(0.0, -1.0, 3.0),
        radius: 1.0,
        color: ppm::Color { r: 255, g: 0, b: 0 },
    };

    let ball_b = Sphere {
        center: Vec3::new(2.0, 0.0, 4.0),
        radius: 1.0,
        color: ppm::Color { r: 0, g: 0, b: 255 },
    };

    let ball_c = Sphere {
        center: Vec3::new(-2.0, 0.0, 4.0),
        radius: 1.0,
        color: ppm::Color { r: 0, g: 255, b: 0 },
    };

    let scene = vec![ball_a, ball_b, ball_c];

    let o = PPMImage::new(600, 600, String::from("output"));

    o.save().expect("não foi possível criar o arquivo");
}
