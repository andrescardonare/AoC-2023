use std::fs;
use std::time::Instant;
use std::string::String;

const RED_B: i32 = 12;
const GREEN_B: i32 = 13;
const BLUE_B: i32 = 14;

fn main() {
    let now = Instant::now();
    let input = fs::read_to_string("src/input_d2.txt")
        .expect("Something went wrong reading the file");
    
    let output1 = game_check(input.clone()); // part 1
    let output2 = power_check(input); // part 2

    println!("Execution time: {}µs", now.elapsed().as_micros());
    println!("The total sum of part 1 numbers is: {output1}");
    println!("The total sum of part 2 numbers is: {output2}");
}

fn game_check(input: String) -> i32 {
    let balls = [(RED_B, "red"), (GREEN_B, "green"), (BLUE_B, "blue")];
    let mut output = 0;
    let mut game = 0;

    'ext: for l in input.lines() {
        let mut line = String::new();
        line.push_str(l);
        line.push_str(",!");
        //println!("line: {}", line);
        game += 1;
        let mut value_str = String::new();
        for char in line.chars() {
            match char {
                ',' | ';' => {

                    let val = value_str.trim().to_string();
                    value_str.clear();

                    let mut iter = val.split_whitespace();

                    let value = iter.next().unwrap().parse::<i32>().unwrap();
                    let color = iter.next().unwrap();
                    //println!("value: {value}, color: {color}");
                    
                    for ball in balls.iter() {
                        if ball.1 == color && value > ball.0 {
                            continue 'ext;
                        }
                    }
                    
                },
                '!' => { output += game; println!("Game: {} passed", game);},
                ':' => { value_str.clear();},
                _ => value_str.push(char),
                  
            }
        }

        
        //break; //endl
    }    

    return output;
}

fn power_check(input: String) -> i32 {
    let mut output = 0;
    let mut game = 0;

    for l in input.lines() {
        let mut line = String::new();
        line.push_str(l);
        line.push_str(",!");
        
        let mut red_v = 1;
        let mut green_v = 1;
        let mut blue_v = 1;

        game += 1;
        let mut value_str = String::new();
        for char in line.chars() {
            match char {
                ',' | ';' => {

                    let val = value_str.trim().to_string();
                    value_str.clear();

                    let mut iter = val.split_whitespace();

                    let value = iter.next().unwrap().parse::<i32>().unwrap();
                    let color = iter.next().unwrap();
                    
                    match color {
                        "red" => {
                            if value > red_v {
                                red_v = value;
                            }
                        },
                        "green" => {
                            if value > green_v {
                                green_v = value;
                            }
                        },
                        "blue" => {
                            if value > blue_v {
                                blue_v = value;
                            }
                        },
                        _ => println!("Error: color not found"),
                    }
                    
                },
                '!' => { 
                    let power = red_v * green_v * blue_v;
                    output += power; 
                    println!("Game {game} power: {power}");
                },
                ':' => { value_str.clear();},
                _ => value_str.push(char),
                  
            }
        }

        
        //break; //endl
    }    

    return output;
}