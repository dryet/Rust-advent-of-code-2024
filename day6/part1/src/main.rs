use std::env;
use std::fs;
fn main() {
    let mut result: u32 = 0;
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut field: Vec<Vec<char>> =
        contents
            .split_ascii_whitespace()
            .fold(Vec::new(), |mut arr, s| {
                arr.push(s.chars().fold(Vec::new(), |mut acc, c| {
                    acc.push(c);
                    acc
                }));
                arr
            });

    let mut visited_positions: Vec<(usize, usize)> = Vec::new();
    let mut escape = false;
    while !escape {
        let y = field.iter().enumerate().fold(0, |mut acc, s| {
            if s.1.iter().any(|c| ['^', '>', 'v', '<'].contains(c)) {
                acc = s.0
            };
            acc
        });
        let x = field[y]
            .iter()
            .position(|c| ['^', '>', 'v', '<'].contains(&c))
            .unwrap();
        match field[y][x] {
            '^' => {
                if y == 0 {
                    escape = true;
                } else {
                    match field[y - 1][x] {
                        '#' => field[y][x] = '>',
                        '.' => {
                            field[y][x] = '.';
                            field[y - 1][x] = '^';
                        }
                        _ => println!("Error"),
                    }
                }
            }
            '>' => {
                if x == (field[0].len() - 1) {
                    escape = true;
                } else {
                    match field[y][x + 1] {
                        '#' => field[y][x] = 'v',
                        '.' => {
                            field[y][x] = '.';
                            field[y][x + 1] = '>';
                        }
                        _ => println!("Error"),
                    }
                }
            }
            'v' => {
                if y == (field.len() - 1) {
                    escape = true;
                } else {
                    match field[y + 1][x] {
                        '#' => field[y][x] = '<',
                        '.' => {
                            field[y][x] = '.';
                            field[y + 1][x] = 'v';
                        }
                        _ => println!("Error"),
                    }
                }
            }
            '<' => {
                if x == 0 {
                    escape = true;
                } else {
                    match field[y][x - 1] {
                        '#' => field[y][x] = '^',
                        '.' => {
                            field[y][x] = '.';
                            field[y][x - 1] = '<';
                        }
                        _ => println!("Error"),
                    }
                }
            }
            _ => println!("Error"),
        }
        let pos = (x, y);
        if !visited_positions.contains(&pos) {
            visited_positions.push(pos);
            result += 1;
        }
    }

    println!("Result is {result}");
}
