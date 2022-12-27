use std::ops::Deref;
use std::time::Instant;

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
use std::path::{Path, PathBuf};
use std::rc::{Rc};

type RGB = [usize; 3];

fn collageify(image:&mut DynamicImage, block_size: u32, averages: &Vec<(Rc<DynamicImage>,RGB)>) {

    let width = block_size * (&image).width() / (&image).height();
    let height = block_size;

    let toi16 = |a: usize| i16::try_from(a).unwrap(); 

    for x in (0..image.width()).step_by(usize::try_from(width).unwrap()){
        for y in (0..image.height()).step_by(usize::try_from(height).unwrap()){  

        let block = &image.clone().crop(x, y, width, height);
        //block.save("test/Block.png").expect("Could not save block image");
        
        let average = &image_maths::image_average(&block).map(toi16);
        let chosen = averages.into_iter().min_by(|(_, a), (_, b)| {
            
            let ai = a.map(toi16);
            let bi = b.map(toi16);

            let aval: i16 = (ai[0]-average[0]).abs() + (ai[1]-average[1]).abs() + (ai[2]-average[2]).abs();
            let bval: i16 = (bi[0]-average[0]).abs() + (bi[1]-average[1]).abs() + (bi[2]-average[2]).abs();

            return aval.cmp(&bval);
            
        }).unwrap()
        .0.deref().resize(width, height, image::imageops::FilterType::Triangle);

        image::imageops::overlay(image, &chosen, x.into(), y.into());
        }
    }
}

fn main() {
    
    println!("Loading files...");
    let paths = std::fs::read_dir("./input/").unwrap();

    println!("Loading files1...");
    let mut files: Vec<PathBuf> = paths.map(|a| a.unwrap().path().to_owned() ).collect();
    files.sort_unstable();
    
    println!("Loading files2...");
    let imgs = &(&files).into_iter().map(|path| (path, Rc::new(image_maths::open_file(path))));
    
    
    println!("Calculating averages...");
    let start = Instant::now();
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
    println!("Averages completed in {} secs", (Instant::now() - start).as_secs());
    
    println!("Processing...");

    const BLOCK_SIZE: u32 = 35;

    for i in 0..averages.len() {

        let pathstr = format!("./result/{}.png", i);
        let path = Path::new(&pathstr);

        if path.exists() {
            print!("Skipping {}", i);
            continue;
        }

        let mut tmpimage = averages[i].0.deref().clone();

        let start = Instant::now();
        collageify(&mut tmpimage, BLOCK_SIZE, &averages);
        println!("Frame {} completed in {} secs", BLOCK_SIZE, (Instant::now() - start).as_secs());

        averages[i].0.deref().save(path).unwrap();
    }

    //for image in imgs {


}
