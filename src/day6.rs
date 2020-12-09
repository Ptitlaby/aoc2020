use crate::utils::read_lines;
use std::collections::HashSet;

pub(crate) fn main() {


    // Tests
    let filename_ex = "B:\\Dev\\Rust\\projects\\aoc2020\\input\\6_ex.txt";
    assert_eq!(p1(filename_ex),11);
    assert_eq!(p2(filename_ex),6);

    // p1
    let filename = "B:\\Dev\\Rust\\projects\\aoc2020\\input\\6.txt";
    let r_p1 = p1(filename);
    println!("Total for part1 is {}",r_p1);
    let r_p2 = p2(filename);
    println!("Total for part1 is {}",r_p2);

}



fn p1(filepath: &str) -> i32{
    let mut total:i32 = 0;
    let mut cur_answers = HashSet::new();
    match read_lines(filepath) {
        Ok(lines) => {
            for line in lines {
                if let Ok(c) = line {
                    if c.is_empty()
                    {
                        total += cur_answers.len() as i32;
                        cur_answers.clear();
                    }
                    else {
                        for char in c.chars()
                        {
                            cur_answers.insert(char);
                        }
                    }
                } else {
                    println!("Couldn't read line [{:?}]",line);
                }
            }
            total += cur_answers.len() as i32;

        }
        Err(..) => {
            println!("Couldn't read filepath [{}]",filepath);
        }
    }
    return total;
}

fn p2(filepath: &str) -> i32{
    let mut total:i32 = 0;
    let mut cur_answers = HashSet::new();
    for c in 'a'..='z'
    {
        cur_answers.insert(c);
    }
    let cur_answers_init = cur_answers.clone();
    match read_lines(filepath) {
        Ok(lines) => {
            for line in lines {
                if let Ok(c) = line {
                    if c.is_empty()
                    {
                        total += cur_answers.len() as i32;
                        cur_answers = cur_answers_init.clone();
                    }
                    else {
                        let cur_line_answers : HashSet<char> = c.chars().collect();
                        //println!("{:?}",cur_answers);
                        //println!("{:?}",cur_line_answers);
                        cur_answers = cur_answers.intersection(&cur_line_answers).map(|&x|x).collect();
                        //println!("{:?}",cur_answers);
                    }
                } else {
                    println!("Couldn't read line [{:?}]",line);
                }
            }
            total += cur_answers.len() as i32;

        }
        Err(..) => {
            println!("Couldn't read filepath [{}]",filepath);
        }
    }
    return total;
}