use std::ops::Deref;

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
use std::path::{PathBuf};
use std::rc::{Rc};

fn main() {
    type RGB = [usize; 3];
    
    println!("Loading files...");
    let paths = std::fs::read_dir("./steamed-hams/").unwrap();
    println!("Loading files1...");
    let files: &Vec<PathBuf> = &paths.map(|a| a.unwrap().path().to_owned() ).collect();

    println!("Loading files2...");
    let imgs = &files.into_iter().map(|path| (path, Rc::new(image_maths::open_file(path))));

    println!("Calculating averages...");
    let averages: Vec<(Rc<DynamicImage>,RGB)> = imgs.to_owned().into_iter().map(|(path, image)| {
        
        let cache_parent = &path.parent().unwrap().parent().unwrap().join(".cache");

        if !cache_parent.exists() {
            std::fs::create_dir(cache_parent).unwrap();
        }

        let cache_path = cache_parent.join(format!("{}.avg", path.file_name().unwrap().to_str().unwrap()));

        if cache_path.exists() {
            let average_file = std::fs::read(cache_path).expect("Can not read file");

            let average: RGB = serde_pickle::from_iter(average_file.into_iter(), Default::default()).expect("Invalid file format");

            return (image, average);
        }

        let average = image_maths::image_average(&image);

        let pickled = serde_pickle::to_vec(&average, Default::default()).expect("Couldent pickle :(");
        std::fs::write(cache_path, pickled).expect("Failed to write to file");

        return (image, average);
    }
    ).collect();
    
    println!("Processing...");

    const BLOCK_SIZE: u32 = 200;

    //for img in imgs {

    let mut img = averages[0].0.deref().to_owned();

    let width = BLOCK_SIZE * (&img).height() / (&img).width();
    let height = BLOCK_SIZE;
    let averagess = &averages.to_owned();

    for x in (0..img.width()-width).step_by(usize::try_from(width).unwrap()){
        for y in (0..img.height()-height).step_by(usize::try_from(height).unwrap()){  

        let block = &img.clone().crop(x, y, width, height);
        //block.save("test/Block.png").expect("Could not save block image");
        
        let average = &image_maths::image_average(&block);
        let chosen = averagess.into_iter().min_by(|(_, a), (_, b)| {
            
            let aval = a[0]-average[0] + a[1]-average[1] + a[2]-average[2];
            let bval = b[0]-average[0] + b[1]-average[1] + b[2]-average[2];
            return aval.cmp(&bval);
            
        }).unwrap()
        .0.deref().resize(width, height, image::imageops::FilterType::Triangle);

        image::imageops::overlay(&mut img, &chosen, x.into(), y.into());

        }
    }
    img.save("test/overlayed.png").unwrap();

}
