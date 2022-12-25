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

fn print(thing: &str) {
    println!("{}",thing);
}

fn main() {
    print("Hello, world!");
}
