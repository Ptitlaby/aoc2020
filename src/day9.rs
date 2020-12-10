use crate::utils::read_lines;
use std::collections::{VecDeque};
use std::process::exit;


pub(crate) fn main() {


    // Tests
    let filename_ex = "B:\\Dev\\Rust\\projects\\aoc2020\\input\\9_ex.txt";
    assert_eq!(p1(filename_ex,5),127);

    let filename_ex = "B:\\Dev\\Rust\\projects\\aoc2020\\input\\9_ex2.txt";
    assert_eq!(p2(filename_ex,127),62);
    println!("Test passed");

    let filename = "B:\\Dev\\Rust\\projects\\aoc2020\\input\\9.txt";
    let ret = p1(filename,25);
    println!("{} is not the sum of two numbers in the past 25 numbers",ret);

    let ret2 = p2(filename,ret);
    println!("{} is the second number searched",ret2);
    // test fifo 25
    /*
    let mut queue : LimitedFifo = LimitedFifo{ queue: Default::default(), limit: 0 };
    queue.with_capacity(5);
    queue.add(0);
    println!("{:?}",queue);
    queue.add(1);
    println!("{:?}",queue);
    queue.add(2);
    println!("{:?}",queue);
    queue.add(3);
    println!("{:?}",queue);
    queue.add(4);
    println!("{:?}",queue);
    queue.add(5);
    println!("{:?}",queue);
    queue.add(6);
    println!("{:?}",queue);
    queue.add(7);
    println!("{:?}",queue);
    let ret = queue.contains_with_sum(2);
    println!("{}", ret);
    */
}

fn p2(filepath: &str, needle: i64) -> i64 {
    let mut queue: LimitedSumFifo = LimitedSumFifo{
        queue : Default::default(),
        sum_limit: needle
    };

    if let Ok(lines) = read_lines(filepath) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(c) = line {
                let element: i64 = c.parse::<i64>().unwrap();
                if ! queue.add(element)
                {
                    return queue.get_results();
                }
            }
        }
    }
    return 0
}

#[derive(Clone, Debug)]
struct LimitedSumFifo{
    queue: VecDeque<i64>,
    sum_limit: i64
}
impl LimitedSumFifo{
    fn add(&mut self, e: i64)-> bool{
        let mut sum = self.queue.iter().sum::<i64>();
        while sum > self.sum_limit
        {
            self.queue.pop_front();
            sum = self.queue.iter().sum::<i64>();
        }

        //println!("adding {} to the queue {:?}. Current sum is {}",e,self.queue,sum);
        if sum == self.sum_limit
        {
            return false
        }
        else {
            self.queue.push_back(e);
            return true
        }

    }

    fn get_results(&mut self) -> i64{
        return self.queue.iter().min().unwrap() + self.queue.iter().max().unwrap();
    }
}


fn p1(filepath: &str, preamble_size: usize) -> i64{
    let mut queue : LimitedFifo  =  LimitedFifo{ queue: Default::default(), limit: 0 };
    queue.with_capacity(preamble_size);
    let mut count_lines = 0;
    if let Ok(lines) = read_lines(filepath) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(c) = line {
                let element :i64 = c.parse::<i64>().unwrap();
                count_lines += 1;
                if count_lines > preamble_size
                {
                    if ! queue.contains_with_sum(element)
                    {
                        return element
                    }
                }
                queue.add(element);

            }
        }
    }
    return 0
}


#[derive(Clone, Debug)]
struct LimitedFifo{
    queue: VecDeque<i64>,
    limit: usize
}
impl LimitedFifo{
    fn add(&mut self, e: i64){
        self.queue.push_back(e);
        if self.queue.len() > self.limit
        {
            self.queue.pop_front();
        }
    }

    fn with_capacity(&mut self, size: usize){
        self.queue = VecDeque::with_capacity(size);
        self.limit = size;
    }

    fn contains_with_sum(&self, needle : i64) -> bool{
        if self.queue.is_empty()
        {
            return false;
        }
        //println!("searching {} in queue {:?}",needle, self.queue);

        for i in 0..&self.queue.len()-0
        {
            let val1:&i64 = self.queue.get(i).unwrap();
            if val1 > &needle
            {
                continue
            }
            for j in 1..&self.queue.len()-0
            {
                let val2:&i64 = self.queue.get(j).unwrap();
                if val2 > &needle
                {
                    continue
                }
                //println!("comparing {} with {} and {}. Sum is {}",needle,val1,val2,val1+val2);
                if val1 == val2
                {
                    continue;
                }
                if needle == (val1 + val2)
                {
                    return true;
                }
            }
        }
        return false;
    }
}