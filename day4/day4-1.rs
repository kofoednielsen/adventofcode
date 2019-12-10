fn criteria(num: &i32) -> bool {
    let digits: &str = &num.to_string();
    let mut prev = digits.chars().nth(0).unwrap();
    let mut adjecent = false;
    let mut decreasing = true; 
    for c in digits[1..].chars() {
        if c == prev {
            adjecent = true;
        }
        if c < prev {
            decreasing = false;
        }
        prev = c;
    }
    return adjecent && decreasing
}
fn main() {
    let combs = (109165..576723).filter(|value| criteria(&value))
                                .collect::<Vec<i32>>()
                                .len();
    println!("combinations: {}", combs); 
}
