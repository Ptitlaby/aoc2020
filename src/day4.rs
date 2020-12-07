use crate::utils::read_lines;
use regex::Regex;

pub(crate) fn main() {

    // Tests
    assert_eq!(read_passports("B:\\Dev\\Rust\\projects\\aoc2020\\input\\4_valid.txt"), 4);
    assert_eq!(read_passports("B:\\Dev\\Rust\\projects\\aoc2020\\input\\4_invalid.txt"), 0);
    // One try
    let filename = "B:\\Dev\\Rust\\projects\\aoc2020\\input\\4.txt";
    read_passports(filename);
}

fn read_passports(filepath: &str) -> i32{
    return if let Ok(lines) = read_lines(filepath) {
        let mut cur_passport = Vec::new();
        let mut c_valid = 0;
        let mut c_invalid = 0;

        for line in lines {
            if let Ok(c) = line {
                if c.is_empty()
                {
                    let final_passport = cur_passport.join(" ");
                    cur_passport.clear();
                    if validate_passports(&final_passport)
                    {
                        c_valid += 1;
                    }
                    else {
                        c_invalid +=1;

                    }
                }
                else
                {
                    cur_passport.push(c);
                }

            } else {
                println!("Line reading failed :(");
            }
        }
        // Last line
        if cur_passport.len() > 0
        {
            if validate_passports(&cur_passport.join(" "))
            {
                c_valid += 1;
            }
            else {
                c_invalid +=1;

            }
        }

        println!("{} valid, {} invalids, {} total",c_valid,c_invalid,c_valid+c_invalid);
        c_valid
    } else {
        println!("Error when reading the file {}", filepath);
        0
    }
}

fn validate_passports(passport : &str) -> bool{
    // Initialize regex only once
    lazy_static!{
        static ref RE_BYR :Regex = Regex::new(r"byr:(\d{4})").unwrap();
        static ref RE_IYR :Regex = Regex::new(r"iyr:(\d{4})").unwrap();
        static ref RE_EYR :Regex = Regex::new(r"eyr:(\d{4})").unwrap();
        static ref RE_HGT :Regex = Regex::new(r"hgt:(\d+)(cm|in)").unwrap();
        static ref RE_HCL :Regex = Regex::new(r"hcl:(#[0-9a-f]{6})").unwrap();
        static ref RE_ECL :Regex = Regex::new(r"ecl:(amb|blu|brn|gry|grn|hzl|oth)").unwrap();
        static ref RE_PID :Regex = Regex::new(r"pid:([0-9]{9})(\s|$)").unwrap();
    }

    if RE_BYR.is_match(passport)
    {
        let caps = RE_BYR.captures(passport).unwrap();
        let byr :i32= caps.get(1).map_or(0, |m| m.as_str().parse().unwrap());
        if byr < 1920 || byr > 2002
        {
            //println!("Wrong birth year. Value[{}] for passport {}",byr,passport);
            return false;
        }
        else {
            //print!("BYR[{}] ",byr)
        }
    }
    else {
        //println!("No BYR for passport {}",passport);
        return false;
    }

    if RE_IYR.is_match(passport)
    {
        let caps = RE_IYR.captures(passport).unwrap();
        let iyr :i32= caps.get(1).map_or(0, |m| m.as_str().parse().unwrap());
        if iyr < 2010 || iyr > 2020
        {
            //println!("Wrong iyr. Value[{}] for passport {}",iyr,passport);
            return false;
        }
        else {
            //print!("IYR[{}] ",iyr)
        }
    }
    else {
        //println!("No IYR for passport {}",passport);
        return false;
    }
    if RE_EYR.is_match(passport)
    {
        let caps = RE_EYR.captures(passport).unwrap();
        let eyr:i32= caps.get(1).map_or(0, |m| m.as_str().parse().unwrap());
        if eyr < 2020 || eyr > 2030
        {
            //println!("Wrong eyr. Value[{}] for passport {}",eyr,passport);
            return false;
        }
        else {
            //print!("EYR[{}] ",eyr)
        }
    }
    else {
        //println!("No EYR for passport {}",passport);
        return false;
    }
    if RE_HGT.is_match(passport)
    {
        let caps = RE_HGT.captures(passport).unwrap();
        let h:i32= caps.get(1).map_or(0, |m| m.as_str().parse().unwrap());
        let unit = caps.get(2).map_or("err", |m| m.as_str());

        if (unit =="err") || (unit == "in" && (h < 59 || h > 76)) || (unit == "cm" && (h < 150 || h > 193))
        {
            //println!("Wrong height. Unit[{}] Value[{}] for passport {}",unit,h,passport);
            return false;
        }
        else {
            //print!("HGT[{}{}] ",h,unit)
        }
    }
    else {
        //println!("No HGT for passport {}",passport);
        return false;
    }

    if RE_HCL.is_match(passport)
    {
        let caps = RE_HCL.captures(passport).unwrap();
        let hcl = caps.get(1).map_or("err", |m| m.as_str());
        //print!("HCL[{}] ",hcl);
    }
    else
    {
        //println!("Wrong HCL for passport {}",passport);
        return false;
    }

    if RE_ECL.is_match(passport)
    {
        let caps = RE_ECL.captures(passport).unwrap();
        let ecl = caps.get(1).map_or("err", |m| m.as_str());
        //print!("ECL[{}] ",ecl);
    }
    else
    {
        //println!("Wrong ECL for passport {}",passport);
        return false;
    }


    if RE_PID.is_match(passport)
    {
        let caps = RE_PID.captures(passport).unwrap();
        let pid = caps.get(1).map_or("err", |m| m.as_str());
        //print!("PID[{}] ",pid);
    }
    else
    {
        println!("Wrong PID for passport : {}",passport);
        return false;
    }

    //println!("Valid passport [{}]",passport);
    //println!("{}",passport);
    return true;
}