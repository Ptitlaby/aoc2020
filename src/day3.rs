use crate::utils::read_lines;

pub(crate) fn main() {
    let filename = "B:\\Dev\\Rust\\projects\\aoc2020\\input\\3.txt";
    println!("filename is {}",filename);

    // One try
    check_slope(filename, 3, 1);

    // Many tries
    let tries: Vec<(i64,i64)> = vec![(1,1),(3,1),(5,1),(7,1),(1,2)];
    let result:Vec<i64> = tries.iter().map(|&t| check_slope(filename,t.0,t.1)).collect();
    println!("For the second tests, the product is {}",result.iter().product::<i64>());
}

fn check_slope(puzzle_path : &str, horizontal_step:i64, vertical_step:i64) -> i64{
    return if let Ok(lines) = read_lines(puzzle_path) {
        // Consumes the iterator, returns an (Optional) String
        let mut c_trees = 0;
        let mut cur_lines_skip = vertical_step; // Don't skip first line
        let mut cur_horizontal_pos = 0;

        for line in lines {
            if let Ok(c) = line {
                if cur_lines_skip < vertical_step - 1
                {
                    cur_lines_skip += 1;
                    continue
                } else {
                    cur_lines_skip = 0;
                }
                if c.chars().nth(cur_horizontal_pos % c.len() as usize).unwrap() == '#'
                {
                    c_trees += 1;
                }
                cur_horizontal_pos += horizontal_step as usize;
            } else {
                //println!("Line reading failed :(");
            }
        }
        println!("For the travel ({},{}), {} trees were hit.",horizontal_step,vertical_step, c_trees);
        c_trees
    } else {
        println!("Error when reading the file {}", puzzle_path);
        0
    }
}