use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn get_input(lists_side_by_side: &str) -> (Vec<u32>, Vec<u32>) {
    let mut input1: Vec<u32> = Vec::new();
    let mut input2: Vec<u32> = Vec::new();
    
    for line in lists_side_by_side.lines() {
        let parts: Vec<_> = line.split("   ").collect();
        input1.push(parts[0].parse::<u32>().unwrap());
        input2.push(parts[1].parse::<u32>().unwrap());
    }

    return (input1, input2);
}

#[aoc(day1, part1)]
fn dec1_1(lists: &(Vec<u32>, Vec<u32>)) -> i32 {
    let mut answer: i32 = 0;
    let (mut input1, mut input2) = lists.clone();
    input1.sort();
    input2.sort();
    for (i, num) in input1.into_iter().enumerate(){
        let smth = num as i32 - input2[i] as i32;
        answer += smth.abs();
    }
    return answer
}

#[aoc(day1, part2)]
pub fn dec1_2(lists: &(Vec<u32>, Vec<u32>)) -> u32 {
    let mut answer: u32 = 0;
    let (ref input1, ref input2) = lists;
    for num in input1.iter() {
        answer += num * input2.clone().into_iter().filter(|n| n == num).count() as u32;
    }
    return answer;
}