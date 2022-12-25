use image::{io::Reader as ImageReader, DynamicImage};

pub fn image_sum(image: &DynamicImage) -> [u32; 3] {
    let bytes = image.as_bytes();

    let mut pixels: [u32; 3] = [0, 0, 0];

    for i in 0..bytes.len() {
        pixels[i % 3] += u32::from(bytes[i])
    }

    return pixels;
}

pub fn image_average(image: &DynamicImage) -> [u32; 3] {
    let sums = image_sum(image);
    let mut averages = [0, 0, 0];
    let bytes = image.as_bytes();

    for i in 0..sums.len() {
        averages[i] = sums[i] / u32::try_from(bytes.len() / 3).expect("Too big!!")
    }

    return averages;
}

pub fn open_file(filename: &str) -> DynamicImage {
    return ImageReader::open(filename).expect("Failed to open file").decode().expect("Failed to decode file");
}
