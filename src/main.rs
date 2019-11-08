mod model;
use model::Model;
use tgaimage::{TGAColor, TGAColorRGB, TGAImage};

static RED: TGAColor = TGAColor::Rgb(TGAColorRGB { r: 255, g: 0, b: 0 });
static WHITE: TGAColor = TGAColor::Rgb(TGAColorRGB {
    r: 255,
    g: 255,
    b: 255,
});

static WIDTH: usize = 500;
static HEIGHT: usize = 500;

fn main() {
    let path = std::env::args().nth(1).expect("no path given");
    let mut image = TGAImage::new(WIDTH, HEIGHT, 3);
    let model = Model::new(&path).unwrap();
    let width = WIDTH as f32;
    let height = HEIGHT as f32;

    for i in 0..model.faces.len() {
        let face = &model.faces[i];
        for j in 0..3 {
            let v0 = &model.verts[face[j] as usize - 1];
            let v1 = &model.verts[face[(j + 1) % 3] as usize - 1];
            let x0 = ((v0.x + 1.0) * width / 2.0) as i32;
            let y0 = ((v0.y + 1.0) * height / 2.0) as i32;
            let x1 = ((v1.x + 1.0) * width / 2.0) as i32;
            let y1 = ((v1.y + 1.0) * height / 2.0) as i32;
            line(x0, y0, x1, y1, &mut image, &WHITE);
        }
    }
    image.flip_vertically();
    image.write_tga_file("output.tga", false);
}

fn line(mut x0: i32, mut y0: i32, x1: i32, y1: i32, image: &mut TGAImage, color: &TGAColor) {
    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;
    let mut err2; /* error value e_xy */

    loop {
        image.set(x0 as usize, y0 as usize, color);
        if x0 == x1 && y0 == y1 {
            break;
        }
        err2 = 2 * err;
        if err2 > dy {
            err += dy;
            x0 += sx;
        } /* e_xy+e_x > 0 */
        if err2 < dx {
            err += dx;
            y0 += sy;
        } /* e_xy+e_y < 0 */
    }
}
