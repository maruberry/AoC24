use std::fs::read_to_string;

static FILE_PATH: &str = "src/days/dec1/input.txt";

pub fn dec1_1() {
    let mut answer: i32 = 0;
    let (mut input1, mut input2) = get_input();
    input1.sort();
    input2.sort();
    for (i, num) in input1.into_iter().enumerate(){
        let smth = (num - input2[i]) as i32;
        answer += smth.abs();
    }
    println!("The answer is : {}", answer);
}

pub fn get_input() -> (Vec<u32>, Vec<u32>) {
    let mut input1: Vec<u32> = Vec::new();
    let mut input2: Vec<u32> = Vec::new();
    
    for line in read_to_string(FILE_PATH).unwrap().lines() {
        let parts: Vec<_> = line.split("   ").collect();
        input1.push(parts[0].parse::<u32>().unwrap());
        input2.push(parts[1].parse::<u32>().unwrap());
    }

    return (input1, input2);
}