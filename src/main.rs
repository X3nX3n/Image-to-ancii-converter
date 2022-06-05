use image::io::Reader as ImageReader;
use image::DynamicImage;
use image::GenericImageView;
use image::imageops::FilterType;

fn main() {

    const ANCII_ARR: [char; 10] = ['.', ',', ':', '+', '*', '?', '%', 'S', '#', '@'];

    pub struct AnciiImage {
        bitmap_color: Vec<image::Rgba<u8>>,
        bitmap_gray: Vec<u8>,
        x: u32,
        y: u32,
        ancii_matrix: String,
    }

    impl AnciiImage{
        pub fn new( _bitmap: Vec<image::Rgba<u8>>, _x: u32, _y: u32) -> AnciiImage
        {
            AnciiImage {bitmap_color: _bitmap, bitmap_gray: Vec::new(), x: _x, y: _y, ancii_matrix:String::from("")}
        }
        pub fn convert_to_gray(&mut self)
        {
            for index in 0..self.bitmap_color.len() {
                    let pixel = self.bitmap_color[index as usize];
                    let pixel_gray = (( pixel[0] as u16 +  pixel[1] as u16 +  pixel[2] as u16) / 3) as u8;
                    //println!("{} ", pixel_gray);
                    self.bitmap_gray.push(pixel_gray)
            }   
        }
        pub fn create_ancii_matrix(&mut self)
        {
            for index in 0..self.bitmap_gray.len() {
                let value = self.bitmap_gray[index as usize];
                let index_ancii  = self.ancii_map(value, 0, 255, 0, ANCII_ARR.len() as u8) as usize;
                //println!("{} ", AnciiArr[index_ancii]);
                self.ancii_matrix.push(ANCII_ARR[index_ancii]);
            }   
        }
        fn ancii_map(&mut self, value: u8, start1: u8, stop1: u8, start2: u8, stop2:u8) -> u8
        {
            ((value as f32  / stop1 as f32) * (stop2  - 1) as f32) as u8
        }
    }

    
    let mut img = ImageReader::open("myimage.png").unwrap().decode().unwrap();

    //resize

    let max_with = 350 as f32;

    let new_height = img.height() as f32 / 0.01 * max_with / img.width() as f32;
    img = if img.width() as f32 > max_with || img.height() as f32 > new_height
    {
        img.resize(max_with as u32, new_height as u32, FilterType::Triangle)
    }
    else
    {
        img
    };

    
    let vec = get_pixels(&img);

    //println!("Lenght is: {}", vec.len());

    let mut ancii_image = AnciiImage::new(vec, img.width(), img.height());
    ancii_image.convert_to_gray();
    ancii_image.create_ancii_matrix();

    for i in 0..ancii_image.y
    {
        let slice  = &ancii_image.ancii_matrix[(i * ancii_image.x) as usize.. ((i * ancii_image.x) + ancii_image.x )as usize];
        //print!(" {},{} \n",(i * ancii_image.x),((i * ancii_image.x) + ancii_image.x ));
        print!("{} \n",slice);
    }

    pub fn get_pixels(img: &DynamicImage) -> Vec<image::Rgba<u8>> {
        let vec_len = img.height() * img.width();
        let mut vec: Vec<image::Rgba<u8>> = Vec::with_capacity(vec_len as usize);
        for x in 0..img.height() {
            for y in 0..img.width() {
                let _pixel = img.get_pixel(y, x);
                //println!("Pixels values is: {} {} {}", _pixel[0],_pixel[1], _pixel[2]);
                vec.push(_pixel);
            }
        }
        return vec;
    }
}
