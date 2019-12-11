use std::io::{self, BufRead};
use std::process;


const ADD: i32 = 1; 
const MULT: i32 = 2;
const IN: i32 = 3; 
const OUT: i32 = 4;
const HALT: i32 = 99;


fn getdigit(num: i32, digit: u32) -> i32 {
    (num % 10_i32.pow(digit+1) / 10_i32.pow(digit))
}

fn getvalue(stack: &[i32], position: usize, mode: i32) -> i32 {
    if mode == 1 {
    // Immediate mode
        stack[position]
    } else {
    // Position mode
        stack[stack[position] as usize]
    }
}


fn run(stack: &mut [i32]) {
    let mut ip = 0; // Instruction pointer
    loop {
        let opcode = getdigit(stack[ip], 0) + 10 * getdigit(stack[ip], 1);
        let par1 = getdigit(stack[ip], 2);
        let par2 = getdigit(stack[ip], 3);
        let par3 = getdigit(stack[ip], 4);
        //println!("number: {}, opcode: {}, par1:{}, par2: {}, par3: {}", stack[ip], opcode, par1, par2, par3);
        match opcode {
            HALT => break,
            ADD => {
                stack[stack[ip+3] as usize] = getvalue(stack, ip+1, par1) + getvalue(stack, ip+2, par2);
                ip += 4;
            },            
            MULT => {
                stack[stack[ip+3] as usize] = getvalue(stack, ip+1, par1) * getvalue(stack, ip+2, par2);
                ip += 4;
            },
            IN => {
                let input = io::stdin().lock().lines().next().unwrap().unwrap();
                stack[stack[ip+1] as usize] = input.parse().unwrap(); 
                ip += 2;
            }
            OUT => {
                println!("{}", getvalue(stack, ip+1, par1));
                ip += 2;
            }

            _ => {
                println!("Something went wrong!");
                break;
            }
        }
    }
}


fn main() -> io::Result<()> {
    // read the whole file
    let input = include_str!("input");

    // process the input into a stack
    let mut stack = input 
        .trim()
        .split(',')
        .map(|l| l.parse().unwrap())
        .collect::<Vec<i32>>();

    run(&mut stack);
    /* 
    println!("---------- debug ----------");
    for i in stack {
        println!("{}", i);
    }
    */
    Ok(())
}
