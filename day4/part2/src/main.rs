use std::env;
use std::fs;
use std::cmp;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let iter_rows = contents.split_ascii_whitespace();

    let row_length: i32 = iter_rows.clone().nth(0).unwrap().chars().count().try_into().unwrap();
    let column_length: i32 = iter_rows.clone().count().try_into().unwrap();

    let mut diagonal = String::new();
    diagonal.push('\n');
    for i in 0..(column_length + row_length - 1) {
        for j in cmp::max(0, i - row_length + 1)..cmp::min(column_length, i + 1){
            diagonal.push(iter_rows.clone().nth(j.try_into().unwrap()).unwrap().chars().nth((i-j).try_into().unwrap()).unwrap());
        }
        diagonal.push('\n');
    }   

    let diagonal_str = diagonal.as_str();
    let diagonal_str_rev = &diagonal.chars().rev().collect::<String>(); 

    let mut diagonal_rev = String::new();
    let mut rows_rev = String::new();
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

    let diagonal_rev_str = diagonal_rev.as_str();
    let diagonal_rev_str_rev = &diagonal_rev.chars().rev().collect::<String>(); 

    let target = "MAS";

    let mut pos_diagonal1 = Vec::new();
    let mut pos_diagonal1_rev = Vec::new();
    let mut pos_diagonal2 = Vec::new();
    let mut pos_diagonal2_rev = Vec::new();
    diagonal_str.as_bytes().windows(target.len()).enumerate().for_each(|x | if x.1 == target.as_bytes() {pos_diagonal1.push(x.0)});
    diagonal_str_rev.as_bytes().windows(target.len()).enumerate().for_each(|x | if x.1 == target.as_bytes() {pos_diagonal1_rev.push(x.0)});
    diagonal_rev_str.as_bytes().windows(target.len()).enumerate().for_each(|x | if x.1 == target.as_bytes() {pos_diagonal2.push(x.0)});
    diagonal_rev_str_rev.as_bytes().windows(target.len()).enumerate().for_each(|x | if x.1 == target.as_bytes() {pos_diagonal2_rev.push(x.0)});

    diagonal_str_rev.chars().for_each(|x| println!("{x}"));
    pos_diagonal1_rev.iter().for_each(|&x| println!("{x}"));

    let mut result: u32 = 0;
    for i in pos_diagonal1{
        if pos_diagonal2.contains(&i){
            result += 1;
        }
    }
    
    for i in pos_diagonal1_rev{
        if pos_diagonal2_rev.contains(&i){
            result += 1;
        }
    }
    println!("{result}");
}
