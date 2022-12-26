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
use serde::{Serialize, Deserialize};

fn main() {
    type RGB = [usize; 3];
    
    println!("Loading files...");
    let paths = std::fs::read_dir("./steamed-hams/").unwrap();
    println!("Loading files1...");
    let files: &Vec<String> = &paths.map(|a| a.unwrap().path().to_str().expect("Cant convert into string?").to_owned() ).collect();
    println!("Loading files2...");
    let imgs: &Vec<(&String, DynamicImage)> = &files.into_iter().map(|path| (path, image_maths::open_file(path))).collect();
    println!("Loading files3...");
    let image_arr: &Vec<&DynamicImage> = &imgs.into_iter().map(|(a, b)| b).collect();
    println!("Calculating averages...");
    let averages = &imgs.into_iter().map(|(path, image)| {
        
        let average = image_maths::image_average(&image);
        let path = [path.to_owned().to_owned(), String::from(".avg")].join("");

        let pickled = serde_pickle::to_value(&average).expect("Couldent pickle :(");
        std::fs::write(path, pickled.to_string()).expect("Failed to write to file");

        return average;
    }
    ).collect::<Vec<[usize; 3]>>();
    
    println!("Creating match table...");

    let mut i = 0;
    let exact: HashMap<RGB, usize> = averages.to_owned().into_iter().map(|a| {i+=1; return (a, i);}).collect();

    println!("Processing...");

    const BLOCK_SIZE: u32 = 50;

    //for img in imgs {
        let img = &image_arr[0];
        let block = &img.to_owned().to_owned().crop(0, 0, BLOCK_SIZE, BLOCK_SIZE * (&img).height() / (&img).width());
        block.save("test/Block.png").expect("Could not save block image");

        let index = exact[&image_maths::image_average(&block)];
        let gotten = (&image_arr).get(index).expect("Error fetching image");

        gotten.save("test/Gotten.png").expect("Could not save Gotton image");
    //}

}
