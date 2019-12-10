use std::io::{self,Read};
use std::collections::HashSet; 
const UP: &str = "U";
const DOWN: &str = "D"; const LEFT: &str = "L";
const RIGHT: &str = "R";

// Perform action such as move 5 in dir 'R'. 
// This adds 5 points to the list 
// Example:
// Say we are on (0,0)
// Then the points (0,1), (0,2), (0,3), (0,4) and (0,5) will be added.
fn action(points: &mut Vec<(i32, i32)>, count: &i32, dir: &str) {
    for _ in 0..*count {
        let cur = points.last().cloned().unwrap();
        match dir {
            UP    => points.push((cur.0 + 1, cur.1)),
            DOWN  => points.push((cur.0 - 1, cur.1)), LEFT  => points.push((cur.0, cur.1 - 1)),
            RIGHT => points.push((cur.0, cur.1 + 1)),
            _ => unreachable!()
        }
    }
}

// Decodes and runs all actions
fn run_actions(actions: &Vec<&str>, mut points: Vec<(i32,i32)>) -> Vec<(i32, i32)> {
    for a in actions {
        let dir = &a[0..1]; 
        let count = &a[1..].parse().unwrap();
        action(&mut points, count, dir);
    }
    points
}

fn main() -> io::Result<()> {
    // read the whole file
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Read the two input lines in as two lists
    // actions[0] = line 1
    // actions[1] = line 2 
    let actions: Vec<Vec<&str>> = input.trim()
                       .split('\n')
                       .map(|s| s.split(',').collect())
                       .collect();
    // Define lists for the points to be added to
    let mut a_points: Vec<(i32,i32)> = vec![(0,0)];
    let mut b_points: Vec<(i32,i32)> = vec![(0,0)]; 

    // Get all the points which the lines touch
    a_points = run_actions(&actions[0], a_points);
    b_points = run_actions(&actions[1], b_points);

    // clone the lists into hash sets
    let a_set: HashSet<(i32,i32)> = a_points.clone().into_iter().collect();
    let b_set: HashSet<(i32,i32)> = b_points.clone().into_iter().collect();

    // Find intersections between the two lines
    let common = a_set.intersection(&b_set);

    // Find out how many steps it took to reach the intersections for both lines
    let mut dists = common.map(|&p| a_points.iter().position(|&a| a == p).unwrap() +
                                   b_points.iter().position(|&b| b == p).unwrap())
                          .collect::<Vec<usize>>();

    dists.sort();

    // Print the shortest distance. Element 0 will be (0,0) 
    println!("Shortest distance: {}", dists[1]);
    Ok(())
}
