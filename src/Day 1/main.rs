use std::fs;
use std::time::Instant;
use std::string::String;

fn main() {
    let now = Instant::now();
    let input = fs::read_to_string("input_d1.txt")
        .expect("Something went wrong reading the file");

    let output1 = analyze(&input); // part 1
    let output2 = analyze(reformat(&input).as_str()); // part 2
    
    println!("Execution time: {}µs", now.elapsed().as_micros());
    println!("The total sum of part 1 numbers is: {output1}");
    println!("The total sum of part 2 numbers is: {output2}");
   
}

fn analyze(input: &str) -> i32 { //part 1 
    let mut result:i32 = 0;

    for line in input.lines() {
        let mut first_char = true;
        let mut single_number = true;
        let mut l_result:i32 = 0;
        
        for char in line.chars(){
            if char.is_numeric() {
                if first_char {
                    l_result += char.to_digit(10)
                        .unwrap() as i32 * 10;
                    first_char = false;

                }else{
                    l_result -= l_result % 10;
                    l_result += char.to_digit(10)
                        .unwrap() as i32;
                    single_number = false;
                }
            }
        }
        if single_number { l_result += l_result / 10;}
        result += l_result;
        //println!("{l_result}");

    }
    result
} 

fn reformat (input: &str) -> String{
    let numbers = [("one", "1"), ("two", "2"), ("three", "3"), ("four", "4"), ("five", "5"), ("six", "6"), ("seven", "7"), ("eight", "8"), ("nine", "9")];

    let mut result = String::from("");
    

    for line in input.lines() {
        let mut line_result:String = String::from("");
        for char in line.chars(){
            if !char.is_numeric() {
                line_result.push(char);
                
                for number in numbers.iter() {
                    if line_result.contains(number.0) {
                        result.push_str(number.1);
                        let temp = line_result.pop().unwrap();
                        line_result.clear();
                        line_result.push(temp);
                    }
                }
                
            }else{
                result.push(char);
            }
        }
        result.push_str("\r\n");
    }
    result
}