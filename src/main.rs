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
pub mod image_maths;

use image::{DynamicImage};

fn main() {

    let files = ["half.png", "thing.png"].map(|a| ["test/", a].join(""));

    let imgs = files.map(|a| Box::new(image_maths::open_file(&a)));
    let averages = imgs.map(|a| (image_maths::image_average(&a), a));

    // [[std::vec::Vec<&DynamicImage>; 255]; 3]
    const INIT: std::vec::Vec<Box<DynamicImage>> = vec![];
    const COLOURS: usize = 255;
    const CHANNEL: [std::vec::Vec<Box<DynamicImage>>; COLOURS] = [INIT; COLOURS];
    let mut candidates = [CHANNEL; 3];

    println!("{:?}",candidates);
}
