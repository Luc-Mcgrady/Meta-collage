use image::DynamicImage;
pub mod image_maths;

type RGB = [usize; 3];

pub struct ImageAverage {
    image: DynamicImage,
    thumbnail: DynamicImage,
    averages: [RGB; 9]
}

impl ImageAverage {
    pub fn new (image: DynamicImage, size: u32) -> Self {

        let width = size * image.width() / image.height();
        let height = size;

        let thumbnail = image.resize(width, height, image::imageops::FilterType::Triangle);
        let mut averages = [[0; 3]; 9];

        for i in 0..9 {
            let x = (width / 3) * i % 3;
            let y = (height / 3) * i / 3;
            
            let block = image.crop_imm(x, y, width / 3, height / 3);
            averages[0] = image_maths::image_average(&block);
        }

        return Self { image, thumbnail, averages };
    }

    
    pub fn weight(&self, other: ImageAverage) {
        
    }
}