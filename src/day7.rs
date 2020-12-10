use crate::utils::read_lines;
use std::collections::HashMap;
use regex::Regex;


pub(crate) fn main() {


    // Tests
    let filename_ex = "B:\\Dev\\Rust\\projects\\aoc2020\\input\\7_ex.txt";
    let all_bags_tests = parse_file(filename_ex);
    assert!(can_hold(all_bags_tests.get("bright white").unwrap(), all_bags_tests.get("shiny gold").unwrap(), &all_bags_tests));
    assert!(can_hold(all_bags_tests.get("muted yellow").unwrap(), all_bags_tests.get("shiny gold").unwrap(), &all_bags_tests));
    assert!(can_hold(all_bags_tests.get("dark orange").unwrap(), all_bags_tests.get("shiny gold").unwrap(), &all_bags_tests));
    assert!(can_hold(all_bags_tests.get("light red").unwrap(), all_bags_tests.get("shiny gold").unwrap(), &all_bags_tests));
    assert_eq!(count_hold(all_bags_tests.get("shiny gold").unwrap(), &all_bags_tests), 4);

    // Tests p2
    let filename_ex_2 = "B:\\Dev\\Rust\\projects\\aoc2020\\input\\7_ex_2.txt";
    let all_bags_tests_2 = parse_file(filename_ex_2);
    println!("{:?}",all_bags_tests_2);
    assert_eq!(count_total_bags(all_bags_tests_2.get("shiny gold").unwrap(), &all_bags_tests_2),126);
    println!("All tests passed :)");

    // Real puzzle
    let filename = "B:\\Dev\\Rust\\projects\\aoc2020\\input\\7.txt";
    let all_bags = parse_file(filename);
    let c = count_hold(all_bags.get("shiny gold").unwrap(), &all_bags);
    println!("{} bags can contains at least one shiny gold bag",c);
    let ct = count_total_bags(all_bags.get("shiny gold").unwrap(), &all_bags);
    println!("{} individual bags are required inside my single shiny gold bag",ct);
}

#[derive(Eq, PartialEq, Hash, Debug)]
struct BagContainRule{
    bag: String,
    count: i32,
}

#[derive(Eq, PartialEq, Hash, Debug)]
struct Bag{
    bag_type: String,
    rules : Vec<BagContainRule>,
    total_contained_bags: i32
}
impl Bag{
    fn add_rule(&mut self, rule: BagContainRule)
    {
        self.total_contained_bags += rule.count;
        self.rules.push(rule);
    }
}

fn count_hold(held_bag: &Bag, all_bags: &HashMap<String, Bag>) -> i32{
    let mut ret = 0;
    for x in all_bags.values() {
        if can_hold(x, held_bag, all_bags)
        {
            //println!("{} can hold a {} bag",x.bag_type,held_bag.bag_type);
            ret += 1;
        }
    }
    return ret
}

fn count_total_bags(held_bag: &Bag, all_bags: &HashMap<String, Bag>) -> i32{
    let mut total = held_bag.total_contained_bags;
    for x in &held_bag.rules{
        //println!("For {}, adding the bags from rule {}",held_bag.bag_type,x.bag);
        total += x.count * count_total_bags(all_bags.get(&x.bag).unwrap(),all_bags);
    }
    return total;
}

fn can_hold(holder_bag: &Bag, held_bag: &Bag, all_bags : &HashMap<String, Bag>) -> bool{
    let mut any = false;
    for rule in &holder_bag.rules{
        if rule.bag == held_bag.bag_type
        {
            return true;
        }
        else {
            any |= can_hold(all_bags.get(&rule.bag).unwrap(),held_bag, all_bags);
        }
    }
    return any;
}


fn parse_file(filepath : &str) -> HashMap<String,Bag>{
    let mut bags = HashMap::new();
    match read_lines(filepath) {
        Ok(lines) => {
            for line in lines {
                if let Ok(c) = line {
                    parse_line(&c,&mut bags);
                } else {
                    println!("Couldn't read line [{:?}]",line);
                }
            }
        }
        Err(..) => {
            println!("Couldn't read filepath [{}]",filepath);
        }
    }
    //println!("[{:?}]",bags);
    return bags;
}



fn parse_line(line: &str, existing_bags: &mut HashMap<String, Bag>){
    // Initialize regex only once
    lazy_static!{
        static ref RE_LINE :Regex = Regex::new(r"([a-zA-Z ]+) bags contain ([a-zA-Z0-9, ]*).").unwrap();
        static ref RE_BAGRULE :Regex = Regex::new(r"(\d) ([a-zA-Z ]*) bags?").unwrap();
    }
    if RE_LINE.is_match(line)
    {
        let caps = RE_LINE.captures(line).unwrap();
        let part1 = caps.get(1).map_or("", |m| m.as_str());
        let s_part1 = String::from(part1);
        let part2 = caps.get(2).map_or("", |m| m.as_str());

        // Bag
        if ! existing_bags.contains_key(&s_part1)
        {
            existing_bags.insert(s_part1, Bag{
                bag_type: String::from(part1),
                rules: Vec::new(),
                total_contained_bags: 0
            });
        }


        let split_part2 = part2.split(",");
        for rule in split_part2
        {
            if RE_BAGRULE.is_match(rule)
            {
                let caps_bagrule = RE_BAGRULE.captures(rule).unwrap();
                let number =  caps_bagrule.get(1).map_or(0, |m| m.as_str().parse().unwrap());
                let rule_bag_name_str =  caps_bagrule.get(2).map_or("", |m| m.as_str());
                let rule_bag_name =  String::from(rule_bag_name_str);

                // Creating bag if needed
                if ! existing_bags.contains_key(&rule_bag_name)
                {
                    existing_bags.insert(rule_bag_name, Bag{
                        bag_type: String::from(rule_bag_name_str),
                        rules: Vec::new(),
                        total_contained_bags: 0
                    });
                }
                // Creating rule
                let r = BagContainRule{
                    bag: String::from(rule_bag_name_str),
                    count:number
                };
                let bag = existing_bags.get_mut(part1).unwrap();
                bag.add_rule(r);
            }
        }
    }
    else {
        panic!("Can not parse line !")
    }
}