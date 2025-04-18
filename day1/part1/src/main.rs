use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let numbers: Vec<&str> = contents.split_ascii_whitespace().collect();

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    for (i, &val) in numbers.iter().enumerate() {
        if i % 2 != 0 {
            right.push(val.parse::<i32>().expect("Failed to parse to int"));
        } else {
            left.push(val.parse::<i32>().expect("Failed to parse to int"));
        }
    }

    right.sort();
    left.sort();
    let mut result: u32 = 0;
    for (i, &val) in right.iter().enumerate() {
        result += val.abs_diff(left[i]);
    }
    print!("{result}");
    // Correct result is 1388114
}
