use std::io::{self,Read};

fn calc_fuel(n: i32) -> i32 {
    let fuel = n / 3 - 2;
    if fuel > 0 {
        fuel + calc_fuel(fuel)
    } else { 0 }
}

fn main() -> io::Result<()> {
    // read the whole file
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Split file into lines
    let lines = input.trim().split('\n');
    // Convert to integers
    let numbers = lines.map(|l| l.parse::<i32>().unwrap());
    // Run logic on all numbers and sum the list
    let sum: i32 = numbers.map(|n| calc_fuel(n)).sum();

    println!("{}", sum);
    Ok(())
}
