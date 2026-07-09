use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day4)]
fn get_input(input: &str) -> Vec<Vec<i16>> {
    input.lines()
        .map(|l| l.chars()
            .enumerate()
            .filter(|(_, value)| *value == '@')
            .map(|(i, _)| i as i16)
            .collect())
        .collect()
}

#[aoc(day4, part1)]
pub fn dec4_1(input: &Vec<Vec<i16>>) -> usize{
    let mut answer = 0;

    for row in 0..input.len() {
        for roll in 0..input[row].len() {
            let mut adjacent = input[row].iter().filter(|r| **r == input[row][roll] - 1 || **r == input[row][roll] + 1).count();
            if row != 0 {
                adjacent += input[row - 1].iter().filter(|r| **r >= input[row][roll] - 1 && **r <= input[row][roll] + 1).count();
            }
            if row != input.len() - 1 {
                adjacent += input[row + 1].iter().filter(|r| **r >= input[row][roll] - 1 && **r <= input[row][roll] + 1).count();
            }

            if adjacent < 4 {
                answer += 1;
            }
        }
    }

    answer
}

#[aoc(day4, part2)]
pub fn dec4_2(input: &Vec<Vec<i16>>) -> usize{
    let mut answer = 0;
    let mut map = input.clone();
    let mut row = 0;
    let mut flag = false;
    loop {
    //for row in 0..map.len() {
        for roll in 0..map[row].len() {
            let mut adjacent = map[row].iter().filter(|r| **r == map[row][roll] - 1 || **r == map[row][roll] + 1).count();
            if row != 0 {
                adjacent += map[row - 1].iter().filter(|r| **r >= map[row][roll] - 1 && **r <= map[row][roll] + 1).count();
            }
            if row != map.len() - 1 {
                adjacent += map[row + 1].iter().filter(|r| **r >= map[row][roll] - 1 && **r <= map[row][roll] + 1).count();
            }

            if adjacent < 4 {
                answer += 1;
                map[row].remove(roll);
                flag = true;
                break;
            }
        }
        if !flag {
            row += 1;
        } else if row != 0 {
            row -= 1;
        } else {
            row = 0;
        }
        flag = false;
        if row == map.len() {
            break;
        }
    }

    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str =
        "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.

";
    #[test]
    fn part_1() {
        let parse_input = get_input(INPUT_TEST);
        let answer = dec4_1(&parse_input);
        assert_eq!(13, answer);
    }

    #[test]
    fn part_2() {
        let parse_input = get_input(INPUT_TEST);
        let answer = dec4_2(&parse_input);
        assert_eq!(43, answer);
    }
}