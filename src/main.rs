mod collage;
use std::{path::Path, fs::create_dir, env};

use collage::meta_collage;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        println!("{} <input_dir:path> <output_dir:path> <block_size:int>", &args[0]);
        return
    }

    let input_dir = Path::new(&args[1]);
    if !input_dir.exists() {
        create_dir(input_dir).expect("Failed to load input directory") ;
    }

    let collage_dir = Path::new(&args[2]);
    if !collage_dir.exists() {
        create_dir(collage_dir).expect("Failed to load input directory") ;
    }

    let output_dir = Path::new(&args[3]);
    if !output_dir.exists() {
        create_dir(output_dir).expect("Failed to create output directory");
    }

    let block_size: u32 = args[4].parse().expect("Not a vaild block size");

    meta_collage(input_dir, collage_dir, output_dir, block_size);
}