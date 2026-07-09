use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
fn get_input(input: &str) -> Vec<Vec<usize>> {
    input.lines().map(|l| l.chars().map(|x| x.to_string().parse().unwrap()).collect()).collect()
}

#[aoc(day3, part1)]
pub fn dec3_1(input: &Vec<Vec<usize>>) -> usize{
    let mut answer = 0;

    for line in input {
        let max = line[..line.len() - 1].iter().max().unwrap();
        let pos = line.iter().find(|x| x == &max).unwrap();
        let other = line[pos + 1..].iter().max().unwrap();
        answer += max * 10 + other;
    }

    answer
}

#[aoc(day3, part2)]
pub fn dec3_2(input: &Vec<Vec<usize>>) -> u64{
    let mut answer: u64 = 0;
    let mut pos = 0;
    let len = input[0].len();
    for line in input {
        pos = 0;
        for i in 1..=12 {
            let max = line[pos..len - 12 + i].iter().max().unwrap();
            let pos_new = line[pos..len - 12 + i].iter().position(|x| x == max).unwrap() + 1;
            pos = pos + pos_new;
            //println!("{}", *max as u64 * (10_u64.pow(12 - i as u32)));
            answer += *max as u64 * (10_u64.pow(12 - i as u32));
        }

    }

    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str =
        "987654321111111
811111111111119
234234234234278
818181911112111
";
    #[test]
    fn part_1() {
        let parse_input = get_input(INPUT_TEST);
        let answer = dec3_1(&parse_input);
        assert_eq!(357, answer);
    }

    #[test]
    fn part_2() {
        let parse_input = get_input(INPUT_TEST);
        let answer = dec3_2(&parse_input);
        assert_eq!(3121910778619, answer);
    }

}