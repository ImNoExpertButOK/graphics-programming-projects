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

    // Seguindo a nomenclatura do livro, cw e ch significam
    // canvas width e canvas height, respectivamente.
    let cw: u32 = 600;
    let ch: u32 = 600;

    let o = PPMImage::new(cw, ch, String::from("output"));
    o.save().expect("Não foi possível criar o arquivo");
}
