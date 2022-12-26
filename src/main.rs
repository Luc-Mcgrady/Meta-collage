use std::collections::HashMap;

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
    type RGB = [usize; 3];
    
    println!("Loading files and calculating averages...");
    let paths = std::fs::read_dir("./steamed-hams/").unwrap();
    let files = paths.map(|a| a.unwrap().path().to_str().expect("Cant convert into string?").to_owned() ).collect::<std::vec::Vec<String>>();
    let imgs = &files.into_iter().map(|a| image_maths::open_file(&a)).collect::<Vec<DynamicImage>>();
    let averages = &imgs.into_iter().map(|a| image_maths::image_average(&a)).collect::<Vec<[usize; 3]>>();
    
    println!("Creating match table...");

    let mut i = 0;
    let exact: HashMap<RGB, usize> = averages.to_owned().into_iter().map(|a| {i+=1; return (a, i);}).collect();

    println!("Processing...");

    const BLOCK_SIZE: u32 = 50;

    //for img in imgs {
        let img = &imgs[0];
        let block = &img.clone().crop(0, 0, BLOCK_SIZE, BLOCK_SIZE * (&img).height() / (&img).width());
        block.save("test/Block.png").expect("Could not save block image");

        let index = exact[&image_maths::image_average(&block)];
        let gotten = (&imgs).get(index).expect("Error fetching image");

        gotten.save("test/Gotten.png").expect("Could not save Gotton image");
    //}

}
