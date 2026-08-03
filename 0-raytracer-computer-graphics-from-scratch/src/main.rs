use std::fs::File;
use std::io::{self, BufWriter, Write};

#[derive(Clone, Copy)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

struct PPMImage {
    filename: String,
    width: u32,
    height: u32,
    pixels: Vec<Color>,
}

impl PPMImage {
    fn new(width: u32, height: u32, name: String) -> Self {
        PPMImage {
            filename: name,
            width: width,
            height: height,
            pixels: vec![Color { r: 0, g: 0, b: 0 }; (width * height) as usize],
        }
    }

    #[allow(dead_code)]
    fn pixel(&mut self, x: i32, y: i32, color: Color) {
        // Pixel recebe coordenadas e uma cor. Como estaremos escrevendo
        // a imagem assumindo o ponto (0,0) como o centro da imagem no
        // loop principal, precisamos converter as coordenadas para topo
        // esquerdo, que é o sistema usado pelo formato PPM.

        let screen_x = (self.width as i32 / 2) + x;
        let screen_y = (self.height as i32 / 2) + y;

        let index = (screen_y * self.width as i32 + screen_x) as usize;
        self.pixels[index] = color;
    }

    fn save(&self) -> io::Result<()> {
        let output = File::create(format!("{}.ppm", &self.filename))?;
        let mut writer = BufWriter::new(output);

        writeln!(writer, "P3")?;
        writeln!(writer, "{} {}", self.width, self.height)?;
        writeln!(writer, "255")?;

        for color in &self.pixels {
            writeln!(writer, "{} {} {}", color.r, color.g, color.b)?;
        }

        writer.flush()?;
        Ok(())
    }
}

fn main() {
    let o = PPMImage::new(600, 600, String::from("output"));
    o.save().expect("não foi possível criar o arquivo");
}
