use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut result: u32 = 0;
    contents.split('\n').for_each(|x| { x.split(' ').collect::<Vec<_>>().windows(2).for_each( |x| if ); });

}