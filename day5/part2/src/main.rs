use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let (rules_raw, updates) = contents.split_once("\n\n").unwrap();
    let rules: Vec<&str> = rules_raw.split_ascii_whitespace().collect();
    let mut result: u32 = 0;
    for row in updates.split_ascii_whitespace() {
        let mut numbers: Vec<&str> = row.split(",").collect();
        let mut fixed: bool = false;
        for i in 0..numbers.iter().count() {
            for j in (i + 1)..numbers.iter().count() {
                let mut pair: String = String::new();
                pair.push_str(numbers[j]);
                pair.push_str("|");
                pair.push_str(numbers[i]);
                if rules.contains(&pair.as_str()) {
                    numbers.swap(i, j);
                    fixed = true;
                }
            }
        }
        if fixed {
            result += numbers[numbers.iter().count() / 2].parse::<u32>().unwrap();
        }
    }
    println!("Result is {result}");
}
