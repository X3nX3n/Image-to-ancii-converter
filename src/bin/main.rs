use image::io::Reader as ImageReader;
use ancii_pong::AnciiImage;

fn main() {
    let mut img = ImageReader::open("myimage.png").unwrap().decode().unwrap();
    img = AnciiImage::resize(img);
    let vec = AnciiImage::get_pixels(&img);

    let mut ancii_image = AnciiImage::new(vec, img.width(), img.height());
    ancii_image.convert_to_gray();
    ancii_image.create_ancii_matrix();
    ancii_image.ancii_drow();
}
