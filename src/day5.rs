use crate::utils::read_lines;
use std::collections::HashSet;

pub(crate) fn main() {


    // Tests
    assert_eq!(decode_line("FBFBBFFRLR"),(44,5,357));
    assert_eq!(decode_line("FFFBBBFRRR"),(14,7,119));
    assert_eq!(decode_line("BBFFBBFRLL"),(102,4,820));

    let filename = "B:\\Dev\\Rust\\projects\\aoc2020\\input\\5.txt";
    let highest_seat = p1(filename);
    println!("Highest seat is {}",highest_seat);
    p2(filename);

}

fn decode_line(line: &str) -> (i32, i32, i32)
{
    if line.len() != 10
    {
        println!("Can not parse the seat location. Wrong string length : [{}]",line.len());
        return (0,0,0);
    }
    let mut row_range_start = 0;
    let mut row_range_end = 127;
    let mut column_range_start = 0;
    let mut column_range_end = 7;

    let mut row_range_length = (row_range_end - row_range_start +1)/2 ;
    let mut column_range_length = (column_range_end - column_range_start +1)/2;
    for c in line.chars()
    {
        match c {
            'F' =>{
                row_range_end -= row_range_length;
                row_range_length /= 2;
            },
            'B' => {
                row_range_start += row_range_length;
                row_range_length /= 2;
            },
            'L' => {
                column_range_end -= column_range_length;
                column_range_length /= 2;
            },
            'R'=> {
                column_range_start += column_range_length;
                column_range_length /= 2;
            },
            _ => {
                println!("Can not parse the seat location. Wrong character specified :[{}]",c);
                return (0,0,0);
            }
        }
    }

    return (row_range_start,column_range_end,row_range_start*8+column_range_end);

}


fn p1(filepath: &str) -> i32{
    let mut highest_seat = 0;
    match read_lines(filepath) {
        Ok(lines) => {
            for line in lines {
                if let Ok(c) = line {
                    let (_row, _column, seat) = decode_line(&c);
                    highest_seat = std::cmp::max(highest_seat, seat);
                } else {
                    println!("Couldn't read line [{:?}]",line);
                }
            }
        }
        Err(..) => {
            println!("Couldn't read filepath [{}]",filepath);
        }
    }
    return highest_seat;
}

fn p2(filepath: &str) -> i32{
    let mut parsed_seats = HashSet::new();
    match read_lines(filepath) {
        Ok(lines) => {
            for line in lines {
                if let Ok(c) = line {
                    let (_row, _column, seat) = decode_line(&c);
                    parsed_seats.insert(seat);
                } else {
                    println!("Couldn't read line [{:?}]",line);
                }
            }
        }
        Err(..) => {
            println!("Couldn't read filepath [{}]",filepath);
        }
    }

    let mut existing_seats = HashSet::new();
    for r in 0..127{
        for c in 0..7{
            existing_seats.insert((8*r+c) as i32);
        }
    }

    let candidate_seats = &existing_seats - &parsed_seats;
    for &c_seat in candidate_seats.iter()
    {
        if parsed_seats.contains(&(c_seat + 1)) && parsed_seats.contains(&(c_seat -1))
        {
            println!("Seat {} has two adjacents seats",c_seat);
            return c_seat
        }
    }
    return 0;
}