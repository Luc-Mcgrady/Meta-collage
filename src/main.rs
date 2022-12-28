mod collage;
use std::{path::Path, fs::create_dir_all};

use collage::meta_collage;

fn main() {
    let input_dir = Path::new("./input/");
    create_dir_all(input_dir).expect("Failed to load input directory") ;

    let output_dir = Path::new("./result/");
    std::fs::create_dir_all(output_dir).expect("The \"result\" folder could not be created");

    let block_size = 35;

    meta_collage(input_dir, output_dir, block_size);
}