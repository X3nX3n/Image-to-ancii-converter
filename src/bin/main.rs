use image::io::Reader as ImageReader;
use image::DynamicImage;
use image::GenericImageView;
use image::imageops::FilterType;
use ancii_pong::AnciiImage;

fn main() {

    const OFFSET : f32 = 2.5;
    const MAX_WITH : f32 = 350.0;

    let mut img = ImageReader::open("myimage.png").unwrap().decode().unwrap();
    //resize
    let max_with = 350 as f32;
    let new_height = img.height() as f32 / OFFSET * MAX_WITH / img.width() as f32;
    img = if img.width() as f32 > MAX_WITH || img.height() as f32 > new_height
    {
        img.resize_exact(MAX_WITH as u32, new_height as u32, FilterType::Triangle)
    }
    else
    {
        img
    };
    
    let vec = AnciiImage::get_pixels(&img);

    //println!("Lenght is: {}", vec.len());

    let mut ancii_image = AnciiImage::new(vec, img.width(), img.height());
    ancii_image.convert_to_gray();
    ancii_image.create_ancii_matrix();

    ancii_image.ancii_drow();

}
