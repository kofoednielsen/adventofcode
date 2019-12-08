use std::io::{self,Read};


fn run(stack: &mut [usize]) {
    let mut ip = 0; // Instruction pointer
    loop {
        match stack[ip] {
            99 => break,
            1 => stack[stack[ip+3]] = stack[stack[ip+1]] + stack[stack[ip+2]],
            2 => stack[stack[ip+3]] = stack[stack[ip+1]] * stack[stack[ip+2]],
            _ => println!("Something went wrong!")
        }
        ip += 4;
    }
}


fn main() -> io::Result<()> {
    // read the whole file
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // process the input into a stack
    let mut stack = input 
        .trim()
        .split(',')
        .map(|l| l.parse().unwrap())
        .collect::<Vec<usize>>();

    // Run until program finishes
    run(&mut stack);

    println!("{}", stack[0]);
    Ok(())
}
