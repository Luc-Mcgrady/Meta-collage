// PLAN
// Frames 1, 2 and 3 each have a RGB Value of 4 apart
// Each frame gets its average 8 bit colour value
// (10,10,10), (20,20,20), (30,30,30)
// 255 size array for closest frames given red colour
// Element of array contains vector of candidate arrays
// Given candidate arrays fitness are increased by accuracy factor
// Repeated for other 2 colour channels
// Given frame is used to replace area.
// FIN

use image::{io::Reader as ImageReader, DynamicImage};

fn image_sum(image: &DynamicImage) -> [u32; 3] {
    let bytes = image.as_bytes();

    let mut pixels: [u32; 3] = [0, 0, 0];

    for i in 0..bytes.len() {
        pixels[i % 3] += u32::from(bytes[i])
    }

    return pixels;
}

fn image_average(image: &DynamicImage) -> [u32; 3] {
    let sums = image_sum(image);
    let mut averages = [0, 0, 0];
    let bytes = image.as_bytes();

    for i in 0..sums.len() {
        averages[i] = sums[i] / u32::try_from(bytes.len() / 3).expect("Too big!!")
    }

    return averages;
}

fn open_file(filename: &str) -> DynamicImage {
    return ImageReader::open(filename).expect("Failed to open file").decode().expect("Failed to decode file");
}

fn main() {

    let files = ["half.png", "thing.png"].map(|a| ["test/", a].join(""));

    let imgs = files.map(|a| open_file(&a));

    println!("{:?}", imgs.map(|a| image_average(&a)));
}
