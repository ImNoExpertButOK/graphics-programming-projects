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

    let _scene = vec![ball_a, ball_b, ball_c];

    // Seguindo a nomenclatura do livro, cw e ch significam
    // canvas width e canvas height, respectivamente.
    let cw: u32 = 600;
    let ch: u32 = 600;

    // O livro coloca o centro da imagem como o ponto (0,0)
    // então nosso loop anda em quadrantes em torno do centro,
    // por isso o valor da largura dividido por 2. Como estamos
    // trabalhando com u32 ao negar seus valores podemos dar a
    // volta caso os números sejam muito grandes, então trans-
    // -formamos eles em i64 para termos certeza que caberão.
    let cw_start: i64 = -(cw as i64 / 2);
    let cw_end: i64 = cw as i64 / 2;
    let ch_start: i64 = -(ch as i64 / 2);
    let ch_end: i64 = ch as i64 / 2;

    // Represente a origem dos raios que serão lançados na cena.
    // No livro é chamado de O, mas assim acho mais claro.
    let _origin: Vec3 = Vec3::new(0.0, 0.0, 0.0);

    // Nosso output final.
    let img = PPMImage::new(cw, ch, String::from("output"));

    // O loop principal.
    for _x in cw_start..=cw_end {
        for _y in ch_start..=ch_end {
            // D = CanvasToViewport(x,y)
            // color = TraceRay()
            // pixel(x, y, color)
        }
    }

    img.save().expect("Não foi possível criar o arquivo");
}
