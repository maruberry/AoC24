use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn get_input(lists_side_by_side: &str) -> (Vec<u32>, Vec<u32>) {
    let mut input1: Vec<u32> = Vec::new();
    let mut input2: Vec<u32> = Vec::new();
    
    for line in lists_side_by_side.lines() {
        let parts: Vec<_> = line.split("   ").collect();
        input1.push(parts[0].parse::<u32>().expect("Why is this not a number?"));
        input2.push(parts[1].parse::<u32>().expect("Why is this not a number?"));
    }

    return (input1, input2);
}

#[aoc(day1, part1)]
fn dec1_1(lists: &(Vec<u32>, Vec<u32>)) -> u32 {
    let (mut input1, mut input2) = lists.clone();
    input1.sort();
    input2.sort();
    let answer = input1
                        .iter()
                        .zip(input2.iter())
                        .map(|(a, b)| a.abs_diff(*b))
                        .collect::<Vec<u32>>()
                        .into_iter()
                        .sum::<u32>();
    return answer
}

#[aoc(day1, part2)]
pub fn dec1_2(lists: &(Vec<u32>, Vec<u32>)) -> u32 {
    let (ref input1, ref input2) = lists;
    let input2_iter = input2.iter();
    let answer = input1.iter()
                        .map(|a| a * input2_iter.clone()
                                            .filter(|n| **n == *a)
                                            .count() as u32)
                        .sum();

    return answer;
}