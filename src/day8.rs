use crate::utils::read_lines;
use std::collections::{HashMap, HashSet};
use regex::Regex;
use std::mem;



pub(crate) fn main() {


    // Tests
    let filename_ex = "B:\\Dev\\Rust\\projects\\aoc2020\\input\\8_ex.txt";
    let instructions_ex = parse_file(filename_ex);
    assert_eq!(execute_file(instructions_ex).1,5);
    println!("Test passed");

    let filename_ex_2 = "B:\\Dev\\Rust\\projects\\aoc2020\\input\\8_ex2.txt";
    let instructions_ex2 = parse_file(filename_ex_2);
    let fixed_acc2 = correct_file(instructions_ex2);
    println!("The acc value is {} with the fixed file",fixed_acc2);

    // Real Puzzle
    let filename = "B:\\Dev\\Rust\\projects\\aoc2020\\input\\8.txt";
    let instructions = parse_file(filename);
    let acc = execute_file(instructions);
    println!("The acc value is {}. Was there an infinite loop ? [{}]",acc.1, acc.0);
    
    let instructions = parse_file(filename);
    let fixed_acc = correct_file(instructions);
    println!("The acc value is {} with the fixed file",fixed_acc);



}
#[derive(Clone, Copy, Debug)]
enum Instruction
{
    NOP(i32),
    JMP(i32),
    ACC(i32)
}

fn correct_file(instructions : HashMap<i32, Instruction>) -> i32 {
    for i in 0..instructions.keys().len()
    {
        let instruction = instructions.get(&(i as i32)).unwrap();
        let param = match instruction {
            Instruction::NOP(x) => x,
            Instruction::JMP(x) => x,
            Instruction::ACC(x) => x
        };
        let new_instruction;
        if mem::discriminant(instruction) == mem::discriminant(&Instruction::NOP(0))
        {
            new_instruction = Instruction::JMP(*param);
        } else if mem::discriminant(instruction) == mem::discriminant(&Instruction::JMP(0))
        {
            new_instruction = Instruction::NOP(*param);
        }
        else {
            new_instruction = Instruction::ACC(*param);
        }

        let mut modified = instructions.clone();

        modified.insert(
            i as i32,
            new_instruction
        );
        //println!("Modified instruction {} from {:?} to {:?} ",i,instruction,new_instruction);
        let ret = execute_file(modified);
        if !ret.0
        {
            println!("By modifying the instruction at line {} we avoid an infinite loop", i);
            return ret.1;
        }
        else {
            //println!("The acc value is {}. Was there an infinite loop ? [{}]",ret.1, ret.0);
        }
    }
    return 0;
}

fn execute_file(instructions : HashMap<i32,Instruction>) -> (bool, i32)
{
    let mut done_instructions: HashSet<i32> = HashSet::new();
    let mut cur_instruction_position = 0;
    let mut accu = 0;
    let mut inf_loop = true;
    while ! done_instructions.contains(&cur_instruction_position)
    {
        //println!("--Currently executing instruction at position {}",cur_instruction_position);
        done_instructions.insert(cur_instruction_position);

        // Execute instruction
        let cur_instruction = instructions.get(&cur_instruction_position);
        match cur_instruction {
            Some(Instruction::NOP(_x))=> cur_instruction_position +=1,
            Some(Instruction::JMP(x)) => cur_instruction_position +=x,
            Some(Instruction::ACC(x)) => {
                accu += x;
                cur_instruction_position += 1;
            },
            None => {
                    inf_loop = false;
                    break;
            }
        }
        //println!("--Finished executing instruction. Next position is {}",cur_instruction_position);
    }
    return (inf_loop, accu);
    //return (done_instructions.len() != instructions.keys().len(), accu);
}

fn parse_file(filepath : &str) -> HashMap<i32,Instruction>{
    let mut instructions = HashMap::new();
    let mut line_number = 0;

    lazy_static!{
        static ref RE_LINE :Regex = Regex::new(r"(nop|jmp|acc) ([\+\-0-9]+)").unwrap();
    }
    match read_lines(filepath) {
        Ok(lines) => {
            for line in lines {
                if let Ok(c) = line {
                    if RE_LINE.is_match(&c)
                    {
                        let caps = RE_LINE.captures(&c).unwrap();
                        let instruction = caps.get(1).map_or("", |m| m.as_str());
                        let number =  caps.get(2).map_or(0, |m| m.as_str().parse().unwrap());
                        instructions.insert(line_number,match instruction {
                            "nop" => Instruction::NOP(number),
                            "acc" => Instruction::ACC(number),
                            "jmp" => Instruction::JMP(number),
                            _ => panic!("Error matching instruction")
                        });
                    }
                } else {
                    println!("Couldn't read line [{:?}]",line);
                }
                line_number +=1;
            }
        }
        Err(..) => {
            println!("Couldn't read filepath [{}]",filepath);
        }
    }
    return instructions;
}