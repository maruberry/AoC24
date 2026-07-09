use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
fn get_input(input: &str) -> Vec<Vec<u128>> {

    input.split(",")
        .map(|v| v
            .split("-")
            .map(|n| n.parse::<u128>().unwrap())
            .collect::<Vec<u128>>())
        .collect()
}

#[aoc(day2, part1)]
pub fn dec2_1(input: &Vec<Vec<u128>>) -> u128{

    let mut answer: u128 = 0;

    for (mut range_start, mut range_end) in input.iter().map(|range| (range[0], range[1])) {
        let mut start_str = range_start.to_string();
        let mut end_str = range_end.to_string();
        let mut start_len = start_str.len();
        let mut end_len = end_str.len();
        if start_len == end_len && start_len % 2 == 1 {
            continue;
        } else {
            if start_len % 2 == 1 {
                let mut temp = vec!['0'; start_len + 1 as usize];
                temp[0] = '1';
                start_str = temp.iter().collect::<String>();
                range_start = start_str.parse::<u128>().unwrap();
                start_len = start_str.len();
            }
            if end_len % 2 == 1 {
                let temp = vec!['9'; end_len - 1 as usize];
                end_str = temp.iter().collect::<String>();
                range_end = end_str.parse::<u128>().unwrap();
                end_len = end_str.len();
            }

            let start_firsthalf = start_str[..start_len/2].parse::<u128>().unwrap();
            let end_firsthalf = end_str[..end_len/2].parse::<u128>().unwrap();

            let mid_ans = end_firsthalf - start_firsthalf;
            for n in start_firsthalf..=start_firsthalf + mid_ans {
                let test_num = (n.to_string() + &n.to_string()).parse::<u128>().unwrap();
                if test_num >= range_start && test_num <= range_end {
                    answer += test_num;
                }
            }
        }
    }

    answer
}

#[aoc(day2, part2)]
pub fn dec2_2(input: &Vec<Vec<u128>>) -> u128{
    let mut answer: u128 = 0;

    for (mut range_start, mut range_end) in input.iter().map(|range| (range[0], range[1])) {
        let mut start_str = range_start.to_string();
        let mut end_str = range_end.to_string();
        let mut start_len = start_str.len();
        let mut end_len = end_str.len();

        let mut found: Vec<u128> = vec![];

        //How many repeating characters we are looking for
        for n in 1..end_len/2 + 1 {
            if start_len % n != 0 && end_len % n != 0 {
                continue;
            }

            let mut start_firstpart = start_str[..n].parse::<u128>().unwrap();
            let mut end_firstpart = end_str[..n].parse::<u128>().unwrap();
            if start_len != end_len {
                let mut temp = vec!['0'; n];
                temp[0] = '1';
                start_firstpart = temp.iter().collect::<String>().parse::<u128>().unwrap();
                temp = vec!['9'; n];
                end_firstpart = temp.iter().collect::<String>().parse::<u128>().unwrap();
            }

            for k in start_firstpart..=end_firstpart {
                let test_num_longer = k.to_string().repeat(end_len/n).parse::<u128>().unwrap();
                let test_num_shorter = k.to_string().repeat(start_len/n).parse::<u128>().unwrap();
                if test_num_longer >= range_start && test_num_longer <= range_end && !found.contains(&test_num_longer) && end_len/n > 1{
                    answer += test_num_longer;
                    found.push(test_num_longer);
                }
                if test_num_shorter >= range_start && test_num_shorter <= range_end && !found.contains(&test_num_shorter) && start_len/n > 1 {
                    answer += test_num_shorter;
                    found.push(test_num_shorter);
                }
            }
        }
    }

    answer
}