use crate::ppm;
use crate::vec3;
use vec3::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub color: ppm::Color,
}
