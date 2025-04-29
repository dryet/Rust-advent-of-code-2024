use std::env;
use std::fs;

fn check_xmas(pos: (&usize, &usize), array: &Vec<&str>) -> bool {
    let mut result: bool = false;
    let mut diag1_vec = Vec::new();
    diag1_vec.push(array[*pos.1 + 1].chars().nth(*pos.0 + 1).unwrap());
    diag1_vec.push(array[*pos.1 - 1].chars().nth(*pos.0 - 1).unwrap());
    diag1_vec.sort();
    let mut diag2_vec = Vec::new();
    diag2_vec.push(array[*pos.1 + 1].chars().nth(*pos.0 - 1).unwrap());
    diag2_vec.push(array[*pos.1 - 1].chars().nth(*pos.0 + 1).unwrap());
    diag2_vec.sort();
    let diag1_string = diag1_vec.iter().fold(String::new(), |mut acc, x| {
        acc.push(*x);
        acc
    });
    let diag2_string = diag2_vec.iter().fold(String::new(), |mut acc, x| {
        acc.push(*x);
        acc
    });
    if diag1_string == "MS" && diag2_string == "MS" {
        result = true;
    }
    result
}

fn check_boundary(pos: (&usize, &usize), row_cnt: usize, colomn_cnt: usize) -> bool {
    let mut result: bool = true;
    if [0, row_cnt - 1].contains(&pos.0) || [0, colomn_cnt - 1].contains(&pos.1) {
        result = false;
    }
    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let row_count: usize = contents.split_ascii_whitespace().nth(0).unwrap().len();
    let column_count: usize = contents.split_ascii_whitespace().count();

    let arr_chars: Vec<&str> =
        contents
            .split_ascii_whitespace()
            .fold(Vec::new(), |mut acc: Vec<&str>, y: &str| {
                acc.push(y);
                acc
            });

    let result: usize = arr_chars.iter().enumerate().fold(0, |mut acc, y| {
        for x in y.1.chars().enumerate() {
            if x.1 == 'A' && check_boundary((&x.0, &y.0), row_count, column_count) {
                if check_xmas((&x.0, &y.0), &arr_chars) {
                    acc += 1;
                }
            }
        }
        acc
    });
    println!("{result}");
}
