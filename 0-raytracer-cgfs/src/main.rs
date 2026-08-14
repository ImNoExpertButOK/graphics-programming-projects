mod ppm;
use ppm::PPMImage;
mod vec3;
use vec3::Vec3;
mod shapes;
use shapes::Sphere;

// Seguindo a nomenclatura do livro, cw e ch significam
// canvas width e canvas height, respectivamente.
const DISTANCE_TO_VIEWPORT: f64 = 1.0;
const VW: f64 = 1.0;
const VH: f64 = 1.0;
const CW: i32 = 600;
const CH: i32 = 600;

fn canvas_to_viewport(x: i32, y: i32) -> Vec3 {
    let tmpx = x as f64 * VW / CW as f64;
    let tmpy = y as f64 * VH / CH as f64;
    return Vec3::new(tmpx, tmpy, DISTANCE_TO_VIEWPORT);
}

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

    let _scene = [ball_a, ball_b, ball_c];

    // O livro coloca o centro da imagem como o ponto (0,0)
    // então nosso loop anda em quadrantes em torno do centro,
    // por isso o valor da largura dividido por 2. Como estamos
    // trabalhando com u32 ao negar seus valores podemos dar a
    // volta caso os números sejam muito grandes, então trans-
    // -formamos eles em i64 para termos certeza que caberão.
    let cw_start: i32 = -CW / 2;
    let cw_end: i32 = CW / 2;
    let ch_start: i32 = -CH / 2;
    let ch_end: i32 = CH / 2;

    // Represente a origem dos raios que serão lançados na cena.
    // No livro é chamado de O, mas assim acho mais claro.
    let _origin: Vec3 = Vec3::new(0.0, 0.0, 0.0);

    // Nosso output final.
    let mut img = PPMImage::new(CW, CH, String::from("output"));

    // O loop principal.
    for x in cw_start..cw_end {
        for y in ch_start..ch_end {
            let _direction = canvas_to_viewport(x, y);

            // color = TraceRay()
            let color = ppm::Color {
                r: 128,
                g: 128,
                b: 128,
            };
            img.pixel(x, y, color);
        }
    }

    img.save().expect("Couldn't create file");
}
