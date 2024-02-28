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
        visited.insert(santa_curr.clone());
    }
    println!("{}", visited.len());
    
    santa_curr[0] = 0;
    santa_curr[1] = 0;
    visited.clear();
    let mut iter: i32 = 1;

    for i in input.chars() {
        iter = 1 - iter;
        match iter {
            0 => {
                match i{
                '>' => santa_curr[1] += 1,
                '<' => santa_curr[1] -= 1,
                '^' => santa_curr[0] += 1,
                'v' => santa_curr[0] -= 1,
                _ => {},
                };
                visited.insert(santa_curr.clone());
            },
            1 => {
                match i {
                    '>' => robo_curr[1] += 1,
                    '<' => robo_curr[1] -= 1,
                    '^' => robo_curr[0] += 1,
                    'v' => robo_curr[0] -= 1,
                    _ => {},
                };
                visited.insert(robo_curr.clone());
            },
            _ => {},
        }
    }
    println!("{}",visited.len());
}
