use image::DynamicImage;
pub mod image_maths;

type RGB = [usize; 3];

pub struct ImageAverage {
    pub orig_width: u32,
    pub orig_height: u32, 
    pub thumbnail: DynamicImage,
    averages: [RGB; 9]
}

pub fn calc_averages(image: &DynamicImage) -> [RGB; 9] {
        
    let width = image.width();
    let height = image.height();
    let mut averages = [[0; 3]; 9];

    for i in 0..9 {
        let x = (width / 3) * (i % 3);
        let y = (height / 3) * (i / 3);
        
        let block = image.crop_imm(x, y, width / 3, height / 3);
        //block.save(format!("{}.png", i)).unwrap();
        averages[usize::try_from(i).unwrap()] = image_maths::image_average(&block);
    }

    return averages;
}

impl ImageAverage {

    pub fn new(image: &DynamicImage, size: u32) -> Self {

        let orig_width = image.width();
        let orig_height = image.height();

        let width = size * image.width() / image.height();
        let height = size;
        

        let thumbnail = image.resize(width, height, image::imageops::FilterType::Triangle);
        let averages = calc_averages(&image);

        return Self {orig_width, orig_height, thumbnail, averages };
    }

    pub fn weight(&self, other: [RGB; 9]) -> i16 {

        fn toi16(a: usize) -> i16 { return i16::try_from(a).unwrap(); }

        fn single_average_weight(average: RGB, other_average: RGB) -> i16 {

            let ai = average.map(toi16);
            let ab = other_average.map(toi16);

            return (ai[0]-ab[0]).abs() + (ai[1]-ab[1]).abs() + (ai[2]-ab[2]).abs();
        }

        let mut total = 0;
        for i in 0..9 {
            total += single_average_weight(self.averages[i], other[i])
        }

        return total;
    }
}