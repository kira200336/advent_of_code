use std::fs;
use std::collections::HashSet;

pub fn day_3() {
    let input = fs::read_to_string("input/day_3.txt").unwrap();
    let mut visited: HashSet<Vec<i32>> = HashSet::from([vec![0,0]]);
    let mut santa_curr: Vec<i32> = vec![0,0];
    let mut robo_curr: Vec<i32> = vec![0,0];
    let mut count: i32 = 1;
    
    for i in input.chars() {
        match i {
            '>' => santa_curr[1] += 1,
            '<' => santa_curr[1] -= 1,
            '^' => santa_curr[0] += 1,
            'v' => santa_curr[0] -= 1,
            _ => {},
        };

        if !visited.contains(&santa_curr) {
            count += 1;
            visited.insert(santa_curr.clone());
        }
    }
    println!("{count}");
}
