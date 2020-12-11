use crate::utils::read_lines;
use std::collections::{HashMap};
use std::time::Instant;
use std::cmp::{max,min};
use std::process::exit;

const PUZZLE_SIZE_LINE: usize = 99;
const PUZZLE_SIZE_COLUMNS: usize = 92;

pub(crate) fn main() {
    // Tests p1
    //let filename_ex = "B:\\Dev\\Rust\\projects\\aoc2020\\input\\11_ex.txt";
    //assert_eq!(p1(filename_ex),37);
    //println!("Tests p1 passed");

    // p1
    let filename = "B:\\Dev\\Rust\\projects\\aoc2020\\input\\11.txt";
    let begin_p1 = Instant::now();
    let ret = p1(filename);
    println!("Answer for part 1 is [{}]. Computed in {:?}",ret,begin_p1.elapsed());


    /*
    // Tests p2
    let filename_ex = "B:\\Dev\\Rust\\projects\\aoc2020\\input\\11_ex.txt";
    assert_eq!(p2(filename_ex),8);

    // p2
    let filename = "B:\\Dev\\Rust\\projects\\aoc2020\\input\\11.txt";
    let begin_p2 = Instant::now();
    let ret = p2(filename);
    println!("Answer for part 2 is [{}]. Computed in {:?}",ret,begin_p2.elapsed());

     */

}

fn p1(filepath: &str) -> usize{
    let mut plan = get_puzzle(filepath);
    let mut stab_seating = update_seatings(&plan);
    let ret = count_occupied(&stab_seating);
    //println!("{:?}",plan);
    return ret;
}

fn count_occupied(plan:&[[char; PUZZLE_SIZE_COLUMNS];PUZZLE_SIZE_LINE]) -> usize {
    let mut count = 0;
    for i in 0..PUZZLE_SIZE_LINE
    {
        for j in 0..PUZZLE_SIZE_COLUMNS
        {
            if plan[i][j] == '#'
            {
                count +=1;
            }
        }
    }
    return count;
}

fn update_seatings(plan: &[[char; PUZZLE_SIZE_COLUMNS];PUZZLE_SIZE_LINE]) -> [[char; PUZZLE_SIZE_COLUMNS];PUZZLE_SIZE_LINE]{
    let mut cur_seats = plan.clone();
    let mut new_seats = [['.'; PUZZLE_SIZE_COLUMNS]; PUZZLE_SIZE_LINE];
    let mut any_changed = true;
    let mut c_changed = 0;
    while any_changed {
        //println!("{:?}", cur_seats);
        any_changed = false;
        new_seats = [['.'; PUZZLE_SIZE_COLUMNS]; PUZZLE_SIZE_LINE];
        for i in 0..PUZZLE_SIZE_LINE
        {
            for j in 0..PUZZLE_SIZE_COLUMNS
            {
                new_seats[i][j] = cur_seats[i][j];
                let actual_seat = cur_seats[i][j];
                let c_adj_occu = count_adjacent_occupied(&cur_seats, get_adjacent_seats(i, j));
                if actual_seat == 'L'
                {
                    if c_adj_occu == 0
                    {
                        new_seats[i][j] = '#';
                        any_changed = true;
                        c_changed +=1;
                    }
                }
                if actual_seat == '#'
                {
                    if c_adj_occu >= 4
                    {
                        new_seats[i][j] = 'L';
                        any_changed = true;
                        c_changed +=1;
                    }
                }
            }
        }
        cur_seats = new_seats;

    }
    //println!("{:?}", new_seats);
    return new_seats

}

fn count_adjacent_occupied(seats : &[[char; PUZZLE_SIZE_COLUMNS];PUZZLE_SIZE_LINE], adj : Vec<(usize,usize)>) -> usize{
    let mut count = 0;
    for (i,j) in adj{
        if seats[i][j] == '#'
        {
            count +=1;
        }
    }
    return count;
}

fn get_adjacent_seats(i : usize, j : usize) -> Vec<(usize,usize)>
{
    let mut adj = Vec::new();
    let min_value_i = if i == 0 { 0 } else { i-1 };
    let min_value_j = if j == 0 { 0 } else { j-1 };
    for a in min_value_i..min(i+2,PUZZLE_SIZE_LINE)
    {
        for b in min_value_j..min(j+2,PUZZLE_SIZE_COLUMNS)
        {
            if a == i && b == j { continue };
            adj.push((a,b));
        }
    }
    //println!("({},{}) adj : {:?}",i,j,adj);
    return adj;
}

fn get_puzzle(filepath: &str) -> [[char; PUZZLE_SIZE_COLUMNS];PUZZLE_SIZE_LINE]
{
    let mut seats = [['.'; PUZZLE_SIZE_COLUMNS];PUZZLE_SIZE_LINE];
    let mut l = 0;
    let mut c = 0;

    if let Ok(lines) = read_lines(filepath) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ok_l) = line {
                c = 0;
                for char in ok_l.chars()
                {
                    seats[l][c] = char;
                    c +=1;
                }
            }
            l +=1;
        }

    }
    return seats;
}


fn p2(filepath: &str) -> i64{
    if let Ok(lines) = read_lines(filepath) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(c) = line {
            }
        }
    }
    return 0
}