use std::collections::HashMap;

use std::time::Instant;

mod image_average;
use image_average::image_maths;
use image_average::ImageAverage;

use image::{DynamicImage};
use std::path::{Path, PathBuf};
use std::rc::{Rc};

use self::image_average::calc_averages;

pub fn collage(image: &DynamicImage, block_size: u32, averages: &Vec<Rc<ImageAverage>>) -> DynamicImage {

    let mut new_image = DynamicImage::new_rgb8(image.width(), image.height());

    let width = block_size * averages[0].image.width() / averages[0].image.height();
    let height = block_size;

    for x in (0..image.width()).step_by(usize::try_from(width).unwrap()){
        for y in (0..image.height()).step_by(usize::try_from(height).unwrap()){  
            
            let block = &image.clone().crop(x, y, width, height);
            let average = calc_averages(block);

            let weights = averages.into_iter().map(|a| {
                return (a.weight(average), a);
            });

            let chosen = weights.into_iter().min_by(|(a, _), (b, _)| {

                return a.cmp(&b);

            }).unwrap();

            image::imageops::overlay(&mut new_image, &chosen.1.thumbnail, x.into(), y.into());
            
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
    let averages: &Vec<Rc<ImageAverage>> = &(collage_choices).into_iter().map(|(path, image)| {

        let average = Rc::new(ImageAverage::new(image, block_size));

        return average;
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