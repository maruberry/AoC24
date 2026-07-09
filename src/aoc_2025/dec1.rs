use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn get_input(input: &str) -> Vec<i32> {
    input.lines().map(|l| {if l[..1] == *"R" {l[1..].parse::<i32>().unwrap()} else {-1 * l[1..].parse::<i32>().unwrap()}}).collect()
}

#[aoc(day1, part1)]
fn dec1_1(numbers: &Vec<i32>) -> u32{
    let mut pointer = 50;
    let mut answer = 0;
    for n in numbers {
        pointer += n;
        if pointer == 0 || pointer % 100 == 0 {
            answer += 1;
        }
    }

    answer
}

#[aoc(day1, part2)]
fn dec1_2(numbers: &Vec<i32>) -> u32{
    let mut pointer = 50;
    let mut pointer_new = pointer;
    let mut answer = 0;
    for n in numbers {
        answer += (n / 100).abs() as u32;
        pointer_new += n;
        if (*n > 0 && (pointer_new % 100 < pointer % 100)) || (*n < 0 && (pointer_new % 100 > pointer % 100)) {
            answer += 1;
        }
        pointer = pointer_new;
    }

    answer
}
