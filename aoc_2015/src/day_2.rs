use std::fs;

pub fn day_2() {
    let input = fs::read_to_string("input/day_2.txt").unwrap();
    let binding: Vec<&str> = input.split('\n').collect();
    binding.pop();
    let mut sum: i32 = 0;
    
    for i in binding.iter() {
        let temp: Vec<&str> = i.split('x').collect();
        

        let sub: i32 = 2*temp[0]*temp[1] + 2*temp[0]*temp[2] + 2*temp[1]*temp[2] + temp[0] * temp[1];
        sum += sub;
    }
    println!("{sum}");
}

