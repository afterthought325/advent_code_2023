use std::env;
use std::fs;


fn main() {
    let args: Vec<String> = env::args().collect();

    let input_file_path = &args[1];

    let mut contents = fs::read_to_string(input_file_path).unwrap();
    contents = contents.replace("one","1");
    contents = contents.replace("two","2");
    contents = contents.replace("three","3");
    contents = contents.replace("four","4");
    contents = contents.replace("five","5");
    contents = contents.replace("six","6");
    contents = contents.replace("seven","7");
    contents = contents.replace("eight","8");
    contents = contents.replace("nine","9");
    
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

