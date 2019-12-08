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
    let stack = input 
        .trim()
        .split(',')
        .map(|l| l.parse().unwrap())
        .collect::<Vec<usize>>();

    let goal = 19690720;

    for noun in 0..99 {
        for verb in 0..99 {
            let mut mutstack = (&stack).to_owned();
            mutstack[1] = noun;
            mutstack[2] = verb;
            run(&mut mutstack);
            if mutstack[0] == goal {
                println!("woop!: {}", (100 * noun + verb));
            }
        }
    }
    Ok(())
}
