use std::env;
use std::fs;


#[derive(Debug)]
struct Draw {
    red: usize,
    green: usize,
    blue: usize,
}

const MAX_RED: usize = 12;
const MAX_GREEN: usize = 13;
const MAX_BLUE: usize = 14;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_file_path = &args[1];

    let contents = fs::read_to_string(input_file_path).unwrap();
    let lines = contents.lines();

    let mut games = Vec::<Vec::<Draw>>::new();

    for line in lines {
        let game: Vec<_> = line.split(":")
            .last()
            .unwrap()
            .split(";")
            .collect();
        let mut draws = Vec::<Draw>::new();
        for x in &game {
            let mut draw = Draw{
                red: 0,
                green: 0,
                blue: 0,
            };
            let colors: Vec<_> = x.split(",").collect();
            for color in colors {
                if color.contains("red") {
                    let red = color.trim().split(" ").next().unwrap();
                    draw.red = red.parse().unwrap();
                } else if color.contains("green") {
                    let green = color.trim().split(" ").next().unwrap();
                    draw.green = green.parse().unwrap();
                } else if color.contains("blue") {
                    let blue = color.trim().split(" ").next().unwrap();
                    draw.blue = blue.parse().unwrap();
                }
            }
            draws.push(draw);
        }
        games.push(draws);
    }

    let mut total = 0;
    for (position, game) in games.iter().enumerate() {
        if check_game(game) {
            println!("game {} had {:?}", position + 1, game);
            total = total + position + 1; 
        }
    }

    println!("{}", total);
}

fn check_game( game: &Vec::<Draw>) -> bool {
    for draw in game {
        if draw.red > MAX_RED {
            //println!("red: {} is greater than max: {}", draw.red, MAX_RED);
            return false;
        } else if draw.green > MAX_GREEN {
            //println!("green: {} is greater than max: {}", draw.green, MAX_GREEN);
            return false;
        } else if draw.blue > MAX_BLUE {
            //println!("blue: {} is greater than max: {}", draw.blue, MAX_BLUE);
            return false;
        }
    }
    true
}

