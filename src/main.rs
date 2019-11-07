use std::mem::swap;
use tgaimage::{TGAColor, TGAColorRGB, TGAImage};

static RED: TGAColor = TGAColor::Rgb(TGAColorRGB { r: 255, g: 0, b: 0 });
static WHITE: TGAColor = TGAColor::Rgb(TGAColorRGB {
    r: 255,
    g: 255,
    b: 255,
});

fn main() {
    let mut image = TGAImage::new(100, 100, 3);
    line(13, 20, 80, 40, &mut image, &WHITE);
    line(20, 13, 40, 80, &mut image, &RED);
    line(80, 40, 13, 20, &mut image, &RED);
    image.flip_vertically();
    image.write_tga_file("output.tga", false);
}

fn line(
    mut x0: i32,
    mut y0: i32,
    mut x1: i32,
    mut y1: i32,
    image: &mut TGAImage,
    color: &TGAColor,
) {
    let mut steep = false;
    if (x0 - x1).abs() < (y0 - y1).abs() {
        swap(&mut x0, &mut y0);
        swap(&mut x1, &mut y1);
        steep = true;
    }

    if x0 > x1 {
        swap(&mut x0, &mut x1);
        swap(&mut y0, &mut y1);
    }
    let dx = x1 - x0;
    let dy = y1 - y0;
    let derror = dy.abs() * 2;
    let mut error = 0;
    let mut x = x0 as usize;
    let mut y = y0 as usize;
    while x <= x1 as usize {
        if steep {
            image.set(y, x, color);
        } else {
            image.set(x, y, color);
        }
        error += derror;
        if error > dx {
            y = if y1 > y0 { y + 1 } else { y - 1 };
            error -= dx * 2;
        }
        x += 1;
    }
}
