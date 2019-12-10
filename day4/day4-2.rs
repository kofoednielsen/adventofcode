fn criteria(num: &i32) -> bool {
    let digits: &str = &num.to_string();
    let mut prev = digits.chars().nth(0).unwrap();
    let mut adjecent = false;
    let mut adjcount = 0;
    let mut decreasing = true; 
    for c in digits[1..].chars() {
        if c == prev && !adjecent {
            adjcount += 1;
        }
        else {
            if adjcount == 1 {
                adjecent = true;
            }
            adjcount = 0;
        }
        if c < prev {
            decreasing = false;
        }
        prev = c;
    }
    if adjcount == 1 {
        adjecent = true;
    }
    return adjecent && decreasing
}
fn main() {
    let combs = (109165..576723).filter(|value| criteria(&value))
                                .collect::<Vec<i32>>()
                                .len();
    println!("combinations: {}", combs); 
}
