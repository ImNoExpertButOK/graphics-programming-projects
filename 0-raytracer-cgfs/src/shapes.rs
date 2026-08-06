mod ppm;
use ppm::PPMImage;
mod vec3;
use vec3::Vec3;

pub struct Sphere {
    center: Vec3,
    radius: f64,
    color: ppm::Color,
}
