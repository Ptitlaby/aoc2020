use std::str;
use crate::utils::read_lines;

pub(crate) fn main() {
    let filename = "B:\\Dev\\Rust\\projects\\aoc2020\\input\\2.txt";
    println!("filename is {}",filename);
    if let Ok(lines) = read_lines(filename) {

        let mut c_ok_1 = 0;
        let mut c_ko_1 = 0;

        let mut c_ok_2 = 0;
        let mut c_ko_2 = 0;
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(c) = line {
                // Pushing all numbers into a vector
                if validate_line_v1(&c)
                {
                    c_ok_1 +=1;
                }
                else
                {
                    c_ko_1 +=1;
                }
                if validate_line_v2(&c)
                {
                    c_ok_2 +=1;
                }
                else
                {
                    c_ko_2 +=1;
                }
            } else {
                println!("Line reading failed :(");
            }
        }
        println!("For the first check, {} passwords are valid, {} passwords are invalid. Total number of passwords : {}", c_ok_1, c_ko_1, c_ok_1 + c_ko_1);
        println!("For the second check, {} passwords are valid, {} passwords are invalid. Total number of passwords : {}", c_ok_2, c_ko_2, c_ok_2 + c_ko_2);

    }
    else {
        println!("Error when reading the file {}",filename);
    }
}

fn validate_line_v2(line: &str ) -> bool{
    let split_result: Vec<&str> = line.split(" ").collect();
    if split_result.len() != 3
    {
        println!("Invalid size for password check. received data : {:?}",split_result);
        return false;
    }
    let min_max : Vec<&str> = split_result[0].split("-").collect();
    let pos1 : i32 = min_max[0].parse::<i32>().unwrap()-1;
    let pos2 : i32 = min_max[1].parse::<i32>().unwrap()-1;
    let letter = str::replace(split_result[1], ":","").chars().nth(0).unwrap(); // Converting to char
    let password_chars: Vec<char> = split_result[2].chars().collect();

    return (password_chars[pos1 as usize] == letter) ^ (password_chars[pos2 as usize] == letter);
}


fn validate_line_v1(line: &str ) -> bool{
    let split_result: Vec<&str> = line.split(" ").collect();
    if split_result.len() != 3
    {
        println!("Invalid size for password check. received data : {:?}",split_result);
        return false;
    }
    let min_max : Vec<&str> = split_result[0].split("-").collect();
    let min : i32 = min_max[0].parse::<i32>().unwrap();
    let max : i32 = min_max[1].parse::<i32>().unwrap();
    let letter = str::replace(split_result[1], ":","").chars().nth(0).unwrap(); // Converting to char
    let password = split_result[2];
    let mut c_occur = 0;
    for c in password.chars()
    {
        if c == letter
        {
            c_occur +=1;
        }
    }
    return c_occur >= min && c_occur <= max;
}


