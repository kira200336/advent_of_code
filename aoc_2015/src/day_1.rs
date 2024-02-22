use std::fs;

pub fn day_1() {
    let mut floor: i32 = 0;
    let mut count: i32 = 0;
    let input: String = fs::read_to_string("input/day_1.txt").unwrap();
    
    for i in input.chars() {
        match i {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => {} 
        };
    };

    println!("part one: {floor}");

    floor = 0;

    for i in input.chars() {
        match i {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => {}
        };
        count += 1;
        if floor == -1 {
            println!("part two: {count}");
            break;
        }
    }
}
