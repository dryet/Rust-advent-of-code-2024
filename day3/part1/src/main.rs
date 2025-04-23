use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let result: i32 = contents
        .split("mul(")
        .filter(|&x| x.contains(")"))
        .filter(|&x| x.contains(","))
        .filter(|&x| {
            !x.split(")")
                .nth(0)
                .unwrap()
                .contains(|c: char| (c != ',') && (!c.is_numeric()))
        })
        .map(|x| x.split(")").nth(0).unwrap())
        .map(|x| {
            x.split(",")
                .nth(0)
                .unwrap()
                .parse::<i32>()
                .expect("Failed to parse to int")
                * x.split(",")
                    .nth(1)
                    .unwrap()
                    .parse::<i32>()
                    .expect("Failed to parse to int")
        })
        .reduce(|acc, e| acc + e)
        .unwrap_or(0);

    println!("{result}"); // Result is 166357705
}
