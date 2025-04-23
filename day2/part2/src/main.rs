use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut result = 0;

    for row_string in contents.split('\n') {
        if row_string.is_empty() {
            break;
        }
        let row: Vec<_> = row_string
            .split(' ')
            .map(|x| x.parse::<i32>().expect("Failed to parse to int"))
            .collect();

        if row
            .windows(2)
            .all(|a| a[0] < a[1] && a[0].abs_diff(a[1]) <= 3)
            || row
                .windows(2)
                .all(|a| a[0] > a[1] && a[0].abs_diff(a[1]) <= 3)
        {
            result += 1;
        } else {
            for i in 0..row.iter().count() {
                let mut row_cpy = row.clone();
                row_cpy.remove(i);

                if row_cpy
                    .windows(2)
                    .all(|a| a[0] < a[1] && a[0].abs_diff(a[1]) <= 3)
                    || row_cpy
                        .windows(2)
                        .all(|a| a[0] > a[1] && a[0].abs_diff(a[1]) <= 3)
                {
                    result += 1;
                    break;
                }
            }
        }
    }
    println!("{result}");
}
