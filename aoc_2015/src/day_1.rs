use std::fs
fn day_1 {
    let mut floor: i32 = 0;
    let input: String = fs::read_to_string("input/day_1.txt").unwrap();
    for i in input.chars() {
        match i {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => {} 
        };
    }
    println!("{floor}");
}
