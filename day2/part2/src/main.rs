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
        let mut row: Vec<_> = row_string
            .split(' ')
            .map(|x| x.parse::<i32>().expect("Failed to parse to int"))
            .collect();

        let mut incremented = false;
        if row
            .windows(2)
            .all(|a| a[0] < a[1] && a[0].abs_diff(a[1]) <= 3)
        {
            result += 1;
            incremented = true;
        } else {
            let position = row
                .windows(2)
                .position(|a| a[0] > a[1] || a[0] == a[1] || a[0].abs_diff(a[1]) > 3);

            if position != None {
                let index = position.unwrap();
                let mut temp_row = row.clone();
                temp_row.remove(index);

                if temp_row
                    .windows(2)
                    .all(|a| a[0] < a[1] && a[0].abs_diff(a[1]) <= 3)
                    || temp_row
                        .windows(2)
                        .all(|a| a[0] > a[1] && a[0].abs_diff(a[1]) <= 3)
                {
                    result += 1;
                    incremented = true;
                }
            }
        }

        if !incremented {
            if row
                .windows(2)
                .all(|a| a[0] > a[1] && a[0].abs_diff(a[1]) <= 3)
            {
                result += 1;
            } else {
                let position = row
                    .windows(2)
                    .position(|a| a[0] < a[1] || a[0] == a[1] || a[0].abs_diff(a[1]) > 3);

                if position != None {
                    let index = position.unwrap();
                    row.remove(index);

                    if row
                        .windows(2)
                        .all(|a| a[0] < a[1] && a[0].abs_diff(a[1]) <= 3)
                        || row
                            .windows(2)
                            .all(|a| a[0] > a[1] && a[0].abs_diff(a[1]) <= 3)
                    {
                        result += 1;
                    }
                }
            }
        }

        println!("{result}");
    }
}
