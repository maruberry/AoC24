use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day5)]
fn get_input(input: &str) -> (Vec<Vec<u128>>, Vec<u128>) {
    let (ranges, ingredients) = input.split_once("\n\n").unwrap();
    let ranges_better: Vec<Vec<u128>> = ranges
        .lines()
        .map(|r| r
            .split('-')
            .map(|s| s.parse::<u128>().unwrap())
            .collect())
        .collect();
    let ingredients_better = ingredients.lines().map(|l| l.parse::<u128>().unwrap()).collect();
    (ranges_better, ingredients_better)
}

#[aoc(day5, part1)]
pub fn dec5_1(input: &(Vec<Vec<u128>>, Vec<u128>)) -> u128 {
    let (ranges, ingredients) = input.clone();
    let mut answer = 0;
    for i in ingredients {
        let pos = ranges.iter().position(|r| r[0] <= i && r[1] >= i);
        if pos.is_some() {
            answer += 1;
        }
    }
    answer
}


//ANSWERS
// 356808706084336 Too high
// 389625483225570 Haven't submitted bcs too high
// 356808706084336
// 356808706084353 // Too high
// 356808706084336
// 356808706084336
// 344813017450467

#[aoc(day5, part2)]
pub fn dec5_2(input: &(Vec<Vec<u128>>, Vec<u128>)) -> u128 {
    let (mut ranges, _ingredients) = input.clone();
    ranges.sort();
    let mut dedup_ranges: Vec<Vec<u128>> = vec![];
    for range in ranges.clone() {
        let pos = dedup_ranges.iter()
            .position(|r| r[0] <= range[0] && r[1] >= range[0]);
        if pos.is_some() {
            let i = pos.unwrap();
            if range[0] < dedup_ranges[i][0] {
                dedup_ranges[i][0] = range[0];
            }
            if range[1] > dedup_ranges[i][1] {
                dedup_ranges[i][1] = range[1];
            }
        } else {
            dedup_ranges.push(range);
        }
    }
    println!("{:#?}", dedup_ranges);
    dedup_ranges.iter().map(|r| r[1] - r[0] + 1).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str =
        "3-5
10-14
11-12
12-18

1
5
8
11
17
32
";
    #[test]
    fn part_1() {
        let parse_input = get_input(INPUT_TEST);
        let answer = dec5_1(&parse_input);
        assert_eq!(3, answer);
    }

    #[test]
    fn part_2() {
        let parse_input = get_input(INPUT_TEST);
        let answer = dec5_2(&parse_input);
        assert_eq!(14, answer);
    }

}