use std::collections::HashMap;
use std::fs::{create_dir_all};
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

fn collageify(image: &DynamicImage, block_size: u32, averages: &Vec<(Rc<DynamicImage>,RGB)>) -> DynamicImage {

    let width = block_size * (&image).width() / (&image).height();
    let height = block_size;

    let toi16 = |a: usize| i16::try_from(a).unwrap();
    let mut new_image = DynamicImage::new_rgb8(image.width(), image.height());

    let mut shrunk_cache: HashMap<RGB, DynamicImage> = HashMap::new();
    let mut best_cache:  HashMap<[i16; 3], &(Rc<DynamicImage>, [usize; 3])> = HashMap::new();

    for x in (0..image.width()).step_by(usize::try_from(width).unwrap()){
        for y in (0..image.height()).step_by(usize::try_from(height).unwrap()){  

            let block = &image.clone().crop(x, y, width, height);
            let average = &image_maths::image_average(&block).map(toi16);

            let chosen: &(Rc<DynamicImage>, [usize; 3]);

            if best_cache.contains_key(average) {
                chosen = best_cache[average];
            }
            else {
                chosen = averages.into_iter().min_by(|(_, a), (_, b)| {
                    
                    let ai = a.map(toi16);
                    let bi = b.map(toi16);

                    let aval: i16 = (ai[0]-average[0]).abs() + (ai[1]-average[1]).abs() + (ai[2]-average[2]).abs();
                    let bval: i16 = (bi[0]-average[0]).abs() + (bi[1]-average[1]).abs() + (bi[2]-average[2]).abs();

                    return aval.cmp(&bval);
                    
                }).unwrap();
                best_cache.insert(*average, chosen);
            }

            let image_index = chosen.1;
            if shrunk_cache.contains_key(&image_index) {
                image::imageops::overlay(&mut new_image, &shrunk_cache[&image_index], x.into(), y.into());
            }
            else {
                let shrunk_image = chosen.0.deref().resize(width, height, image::imageops::FilterType::Triangle);
                shrunk_cache.insert(image_index, shrunk_image);
                image::imageops::overlay(&mut new_image, &shrunk_cache[&image_index], x.into(), y.into());
            }
            

        }
    }
    return new_image
}

fn main() {
    
    let input_dir = Path::new("./input/");

    create_dir_all(input_dir).expect("Failed to load input directory") ;

    let paths = std::fs::read_dir(input_dir).expect("Invalid input directory");
    let mut files: Vec<PathBuf> = paths.map(|a| a.unwrap().path().to_owned() ).collect();
    files.sort_unstable();
    let imgs = &(&files).into_iter().map(|path| (path, Rc::new(image_maths::open_file(path))));
    
    println!("Loading files and calculating averages...");
    let start = Instant::now();
    let averages: Vec<(Rc<DynamicImage>,RGB)> = imgs.to_owned().into_iter().map(|(path, image)| {
        
        let cache_parent = &path.parent().unwrap().parent().unwrap().join(".cache");
        std::fs::create_dir_all(cache_parent).unwrap();

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

    const BLOCK_SIZE: u32 = 25;

    for i in 0..averages.len() {

        let result_path = Path::new("./result/");
        std::fs::create_dir_all(result_path).expect("The \"result\" folder could not be created");

        let path = result_path.join(format!("{:0width$}.png", i, width = 5));

        if path.exists() {
            println!("Skipping {}", i);
            continue;
        }

        let start = Instant::now();
        let solved = collageify( averages[i].0.deref(), BLOCK_SIZE, &averages);
        println!("Frame {} completed in {} secs", i, (Instant::now() - start).as_secs());

        solved.save(path).expect("Could not save output");
    }

}
