use crate::utils::read_lines;

pub(crate) fn main() {
    let filename = "B:\\Dev\\Rust\\projects\\aoc2020\\input\\1.1.txt";
    println!("filename is {}",filename);
    if let Ok(lines) = read_lines(filename) {
        // Create a vector with all numbers
        let mut all_numbers:Vec<i32> = Vec::new();

        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(c) = line {
                // Pushing all numbers into a vector
                all_numbers.push(c.parse::<i32>().unwrap());
            } else {
                println!("Not ok :(");
            }
        }

        // First pass with recursive function
        println!("{} numbers were read from the file",all_numbers.len());
        let (x,y) = find_sum(&all_numbers,2020);
        println!("The product of {} and {} is {}",x,y,x*y);
        let (a,b,c) = find_sum_3(&all_numbers,2020);
        println!("The product of {}, {} and {} is {}",a,b,c,a*b*c);

        // Generic function
        let vec_result = find_sum_n(&all_numbers,2020,3);
        println!("{:?} vector found for size 3",vec_result);

        // Manual tests
        let vec_test = vec![1,2,3,4,5,6];
        let vec_result_test = find_sum_n(&vec_test,12,4);
        println!("{:?} vector found for size 4",vec_result_test);

        let vec_test = vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18];
        let vec_result_test = find_sum_n(&vec_test,92,8);
        println!("{:?} vector found for size 8",vec_result_test);
    }
    else {
        println!("Error when reading the file {}",filename);
    }
}

fn find_sum_n(vec_numbers: &[i32], searched_sum:i32, n:i32) -> Vec<i32> {
    if vec_numbers.len() < n as usize
    {
        println!("Couldn't find {} numbers for which the sum is {}", n, searched_sum);
        return Vec::new()
    }

    // Creating a vector with the position of each index
    let mut indexes = Vec::new();
    for i in (0..n).rev() // Storing the indexes in reversed order
    {
        indexes.push(i as usize);
    }

    // Defining a function which check the sum of the current vector
    fn check_sum(vec_numbers : &[i32], indexes : &[usize], searched_sum: i32) -> bool {
        //println!("Checking sum for indexes{:?}",indexes);
        let mut cur_sum = 0;
        for j in 0..indexes.len()
        {
            //println!("-- Adding {} to the sum",vec_numbers[indexes[j]]);
            cur_sum += vec_numbers[indexes[j]];
        }
        //println!("We searched the value {} and the sum is {}",searched_sum,cur_sum);
        return cur_sum == searched_sum
    }

    // Looping while we don't find the sum
    while ! check_sum(vec_numbers, &indexes, searched_sum)
    {
        // The sum is incorrect
        // we push by one the first index which is not at max value
        let mut index_of_index = 0;
        let mut max_loc = vec_numbers.len()-1;
        while indexes[index_of_index] == max_loc
        {
            index_of_index += 1;
            max_loc -= 1;
            if index_of_index > indexes.len() -1
            {
                println!("Not found");
                return Vec::new();
            }
        }
        //println!("We are modifying index at position {}",index_of_index);
        // We know which index to modify
        indexes[index_of_index] = indexes[index_of_index] +1;
        for i in (0..index_of_index).rev()
        {
            //println!("Propagating change to index at position {}",i);
            //println!("The value at position {} is {}",index_of_index,indexes[index_of_index]);
            indexes[i] = indexes[index_of_index]+index_of_index-i;
        }
    }

    // Creating the Vec with the return values
    let mut ret_values = Vec::new();
    for k in 0..indexes.len()
    {
        ret_values.push(vec_numbers[indexes[k]]);
    }
    return ret_values;

}

fn find_sum(vec_numbers: &[i32], sum :i32) -> (i32,i32) {
    if vec_numbers.len() < 2
    {
        println!("Couldn't find two numbers for which the sum is {}",sum);
        return (0,0)
    }
    let start = vec_numbers[0];
    let slice = &vec_numbers[1..];

    for x in slice.iter(){
        if x + start == sum
        {
            println!("Found two numbers which sum is {} : {} and {}",sum,x,start);
            return (*x, start); // star is used to deference the value from the slice
        }
    }
    return find_sum(slice, sum);
}

// Find the three numbers in a vector for which the sum is the number passed in parameter
fn find_sum_3(vec_numbers: &[i32], sum :i32) -> (i32,i32,i32) {
    if vec_numbers.len() < 3
    {
        println!("Couldn't find three numbers for which the sum is {}",sum);
        return (0,0,0)
    }
    let start = vec_numbers[0];
    let slice = &vec_numbers[1..];
    let slice2 = &vec_numbers[2..];

    for x in slice.iter()
    {
        for y in slice2.iter()
        {
            if x + y + start == sum
            {
                println!("Found three numbers which sum is {} : {}, {} and {}",sum,x,y,start);
                return (*x,*y, start); // star is used to deference the value from the slice
            }
        }
    }
    return find_sum_3(slice, sum);
}