use crate::utils::read_lines;
use std::collections::{HashMap};
use std::time::Instant;
use std::borrow::Borrow;

pub(crate) fn main() {


    // Tests p1
    let filename_ex = "B:\\Dev\\Rust\\projects\\aoc2020\\input\\10_ex.txt";
    assert_eq!(p1(filename_ex),35);
    let filename_ex2 = "B:\\Dev\\Rust\\projects\\aoc2020\\input\\10_ex2.txt";
    assert_eq!(p1(filename_ex2),220);
    println!("Tests p1 passed");

    // p1
    let filename = "B:\\Dev\\Rust\\projects\\aoc2020\\input\\10.txt";
    let beginP1 = Instant::now();
    let ret = p1(filename);
    println!("Answer for part 1 is [{}]. Computed in {:?}",ret,beginP1.elapsed());


    // Tests p2
    let filename_ex = "B:\\Dev\\Rust\\projects\\aoc2020\\input\\10_ex.txt";
    assert_eq!(p2(filename_ex),8);


    let filename_ex = "B:\\Dev\\Rust\\projects\\aoc2020\\input\\10_ex3.txt";
    assert_eq!(p2(filename_ex),8);


    let filename_ex2 = "B:\\Dev\\Rust\\projects\\aoc2020\\input\\10_ex2.txt";
    assert_eq!(p2(filename_ex2),19208);
    println!("Tests p2 passed");

    // p2
    let filename = "B:\\Dev\\Rust\\projects\\aoc2020\\input\\10.txt";
    let beginP2 = Instant::now();
    let ret = p2(filename);
    println!("Answer for part 2 is [{}]. Computed in {:?}",ret,beginP2.elapsed());

}


#[derive(Clone, Debug)]
struct Adapter{
    value: i32,
    compatibles_adapters: Vec<i32>,
    nb_comb: i64
}

impl Adapter{
    fn add_compatible_adapter(&mut self, adapt: i32)
    {
        self.compatibles_adapters.push(adapt);
    }

    fn nb_comb(&mut self, all_adapts : &mut HashMap<i32, Adapter>) -> i64{
        if self.nb_comb != -1
        {
            return self.nb_comb;
        }
        //println!("Computing the number of computations for adapt {}",self.value);

        let mut sum = 0;
        for ca in &self.compatibles_adapters
        {
            if ca == &0
            {
                sum +=1;
                continue;
            }
            let mut next_adapt = all_adapts.get(ca).unwrap().clone();
            sum += next_adapt.nb_comb(all_adapts);
            all_adapts.insert(*ca,next_adapt);
            //sum += all_adapts.get_mut(ca).unwrap().nb_comb(all_adapts);
        }
        //println!("Number of combinations for adapt {} : {}",self.value,sum);
        self.nb_comb = sum;
        return sum;
    }

}

fn p2(filepath: &str) -> i64 {
    let mut adapters_values = Vec::new();
    if let Ok(lines) = read_lines(filepath) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(c) = line {
                let adapter: i32 = c.parse::<i32>().unwrap();
                adapters_values.push(adapter);
            }
        }
    }
    adapters_values.sort();
    let device_value = 3 + *adapters_values.iter().max().unwrap();
    adapters_values.push(device_value);
    //println!("Device value is {}",device_value);

    // Creating all adapters
    let mut adapters: HashMap<i32,Adapter> = HashMap::new();
    for adapt_value in &adapters_values
    {
        let mut adapt = Adapter{
            value:*adapt_value,
            compatibles_adapters: vec![],
            nb_comb: -1
        };
        for i in adapt_value-3..*adapt_value
        {
            if adapters.contains_key(&i)
            {
                adapt.add_compatible_adapter(i);
            }
        }
        if adapt_value <= &3
        {
            adapt.add_compatible_adapter(0);
        }
        adapters.insert(*adapt_value,adapt);
    }

    //println!("All adapters created");

    // Finding number of combinations
    //println!("{:?}",adapters);
    let ret = adapters.get(&device_value).unwrap().clone().nb_comb(&mut adapters);
    println!("Number of combination to go to device value {} : {}",device_value,ret);
    return ret;
}

fn p1(filepath: &str) -> i64{
    let mut adapters = Vec::new();
    adapters.push(0);
    if let Ok(lines) = read_lines(filepath) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(c) = line {
                let adapter :i32 = c.parse::<i32>().unwrap();
                adapters.push(adapter);
            }
        }
    }
    adapters.push(3 + *adapters.iter().max().unwrap());
    adapters.sort();
    //println!("{:?}",adapters);
    let mut c_diff1 = 0;
    let mut c_diff3 = 0;
    for i in 1..adapters.len()
    {
        let cur_adapt = adapters.get(i).unwrap();
        let prev_adapt = adapters.get(i-1).unwrap();
        if cur_adapt - prev_adapt == 1
        {
            c_diff1 +=1;
        }
        else if cur_adapt - prev_adapt == 3 {
            c_diff3 +=1;
        }
    }
    //println!("{} {}",c_diff1, c_diff3);
    return c_diff1 * c_diff3;
}