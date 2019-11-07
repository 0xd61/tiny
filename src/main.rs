use tgaimage::{TGAColor, TGAColorRGB, TGAImage};

static RED: TGAColor = TGAColor::Rgb(TGAColorRGB { r: 255, g: 0, b: 0 });
static WHITE: TGAColor = TGAColor::Rgb(TGAColorRGB {
    r: 255,
    g: 255,
    b: 255,
});

fn main() {
    let mut image = TGAImage::new(100, 100, 3);
    image = line(13, 20, 80, 40, image, &WHITE);
    image.write_tga_file("output.tga", false);
}

fn line(
    x0: usize,
    x1: usize,
    y0: usize,
    y1: usize,
    mut image: TGAImage,
    color: &TGAColor,
) -> TGAImage {
    for t in 0..1 {
        let x = x0 + (x1 - x0) * t;
        let y = y0 + (y1 - y0) * t;
        image.set(x, y, color);
    }
    image
}
