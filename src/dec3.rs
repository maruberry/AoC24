use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[aoc_generator(day3, part1)]
fn generate_p1(input: &str) -> Vec<(i32, i32)>{
    return get_nums(input);
}

#[aoc(day3, part1)]
fn dec3_1(input: &Vec<(i32, i32)>) -> i32 {
    let answer: i32 = input.iter().map(|(x, y)| x*y).sum();

    return answer;
}

#[aoc_generator(day3, part2)]
fn generate_p2(input: &str) -> Vec<(i32, i32)>{
    let splitup = input.split("don't()");
    let mut goodparts: Vec<&str> = Vec::new();

    for (i, part) in splitup.enumerate() {
        if i == 0 {
            goodparts.push(part);
            continue;
        }
        let mut mparts: Vec<&str> = part.split("do()").collect();
        mparts.remove(0);
        goodparts.append(&mut mparts);
    }

    let mut nums: Vec<(i32, i32)> = Vec::new();
    for part in goodparts.iter(){
        nums.append(&mut get_nums(*part));
    }

    return nums
}

#[aoc(day3, part2)]
fn dec3_2(input: &Vec<(i32, i32)>) -> i32 {
    let answer: i32 = input.iter().map(|(x, y)| x*y).sum();

    return answer;
}

fn get_nums(input: &str) -> Vec<(i32, i32)> {
    let re = Regex::new(r"mul\((?<x>\d+),(?<y>\d+)\)").unwrap();

    let nums: Vec<(i32, i32)> = re.captures_iter(input).map(|caps| {
        let x = caps.name("x").unwrap().as_str().parse::<i32>().unwrap();
        let y = caps.name("y").unwrap().as_str().parse::<i32>().unwrap();
        (x, y)
    }).collect();
    return nums;
}
