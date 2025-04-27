use std::env;
use std::fs;
use std::cmp;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let iter_rows = contents.split_ascii_whitespace();

    let mut full_string = contents.clone();
    full_string.push_str(&contents.chars().rev().collect::<String>());

    let row_length: i32 = iter_rows.clone().nth(0).unwrap().chars().count().try_into().unwrap();
    let column_length: i32 = iter_rows.clone().count().try_into().unwrap();

    let mut vertical = String::new();
    vertical.push('\n');
    for i in 0..row_length {
        iter_rows.clone().for_each(|x| vertical.push(x.chars().nth(i.try_into().unwrap()).unwrap()));
        vertical.push('\n');
    }

    full_string.push_str(vertical.as_str());
    full_string.push_str(&vertical.chars().rev().collect::<String>());

    let mut diagonal = String::new();
    diagonal.push('\n');
    for i in 0..(column_length + row_length - 1) {
        for j in cmp::max(0, i - row_length + 1)..cmp::min(column_length, i + 1){
            diagonal.push(iter_rows.clone().nth(j.try_into().unwrap()).unwrap().chars().nth((i-j).try_into().unwrap()).unwrap());
        }
        diagonal.push('\n');
    }   

    full_string.push_str(diagonal.as_str());
    full_string.push_str(&diagonal.chars().rev().collect::<String>()); 

    let mut diagonal_rev = String::new();
    let mut rows_rev =String::new();
    rows_rev.push('\n');
    for i in iter_rows {
        rows_rev.push_str(&i.chars().rev().collect::<String>());
        rows_rev.push('\n');
    }
    let iter_rows_rev = rows_rev.split_ascii_whitespace();

    diagonal_rev.push('\n');
    for i in 0..(column_length + row_length - 1) {
        for j in cmp::max(0, i - row_length + 1)..cmp::min(column_length, i + 1){
            diagonal_rev.push(iter_rows_rev.clone().nth(j.try_into().unwrap()).unwrap().chars().nth((i-j).try_into().unwrap()).unwrap());
        }
        diagonal_rev.push('\n');
    }   

    full_string.push_str(diagonal_rev.as_str());
    full_string.push_str(&diagonal_rev.chars().rev().collect::<String>()); 
    

    let target = "XMAS";

    let result = full_string.as_bytes().windows(target.len()).filter(|&x| x == target.as_bytes()).count();

    for i in full_string.chars() {
        println!("{i}");
    }
    println!("{result}");
}
