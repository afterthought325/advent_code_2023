use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_file_path = &args[1];

    let contents = fs::read_to_string(input_file_path).unwrap();
    let lines = contents.lines();

    let mut calibration = 0;
    for line in lines {
        let first_value = line.chars().find(char::is_ascii_digit).unwrap();
        let second_value = line.chars().rfind(char::is_ascii_digit).unwrap();
        let mut combined_value = String::from(first_value);
        combined_value.push(second_value); 
        calibration = calibration + combined_value.parse::<i32>().unwrap();
    }
    println!("{}", calibration);

}

