use std::collections::HashMap;

use std::ops::Deref;
use std::time::Instant;

mod image_average;
use image_average::image_maths;

use image::{DynamicImage};
use std::path::{Path, PathBuf};
use std::rc::{Rc};

type RGB = [usize; 3];

pub fn collage(image: &DynamicImage, block_size: u32, averages: &Vec<(Rc<DynamicImage>,RGB)>) -> DynamicImage {

    let toi16 = |a: usize| i16::try_from(a).unwrap();
    let mut new_image = DynamicImage::new_rgb8(image.width(), image.height());

    let mut shrunk_cache: HashMap<RGB, DynamicImage> = HashMap::new();
    let mut best_cache:  HashMap<[i16; 3], &(Rc<DynamicImage>, [usize; 3])> = HashMap::new();

    let width = block_size * (&averages[0].0).width() / (&averages[0].0).height();
    let height = block_size;

    for x in (0..image.width()).step_by(usize::try_from(width).unwrap()){
        for y in (0..image.height()).step_by(usize::try_from(height).unwrap()){  
            
            let block = &image.clone().crop(x, y, width, height);
            let average = &[0,0,0];

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

pub fn meta_collage(frames_dir: &Path, collage_dir: &Path, output_dir: &Path, block_size: u32) {
    
    let frame_paths = std::fs::read_dir(frames_dir).expect("Invalid input directory");
    let collage_paths = std::fs::read_dir(collage_dir).expect("Invalid collage directory").map(|a| a.unwrap());

    let mut frame_path_strs: Vec<PathBuf> = frame_paths.map(|a| a.unwrap().path().to_owned() ).collect();
    frame_path_strs.sort_unstable();
    let collage_choices = collage_paths.into_iter().map(|path| (path.path(), Rc::new(image_maths::open_file(&path.path()))));
    let frames = frame_path_strs.into_iter().map(|path| image_maths::open_file(&path));

    println!("Loading files and calculating averages...");
    let start = Instant::now();
    let averages: &Vec<(Rc<DynamicImage>,RGB)> = &(collage_choices).into_iter().map(|(path, image)| {
        
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

    for (i, frame) in frames.enumerate() {

        let path = output_dir.join(format!("{:0width$}.png", i, width = 5));

        if path.exists() {
            println!("Skipping {}", i);
            continue;
        }

        let start = Instant::now();
        let solved = collage( &frame, block_size, &averages);
        println!("Frame {} completed in {} secs", i, (Instant::now() - start).as_secs());

        solved.save(path).expect("Could not save output");
    }

}