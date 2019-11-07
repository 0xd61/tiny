use tgaimage::{TGAColor, TGAImage};

pub fn run() {
    let red = TGAColor::rgb(255, 0, 0);
    let mut image = TGAImage::new(100, 100, 24);
    image.set(52, 41, &red);
    image.flip_vertically(); // i want to have the origin at the left bottom corner of the image
    image.write_tga_file("output.tga", false);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
