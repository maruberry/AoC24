use std::fs::read_to_string;

static FILE_PATH: &str = "src/days/dec2/input.txt";

pub fn dec2_1() {
    let mut answer = 0;
    let reports = read_input();

    for report in reports {
        let asc = report.is_sorted_by(|a, b| a < b && a+3 >= *b);
        let desc = report.is_sorted_by(|a, b| a > b && *a <= b+3);

        println!("{:?}",report);
        if asc || desc {
            answer += 1;
        }
    }
    println!("Day 2 part 1 answer is: {}", answer);
}

pub fn read_input() -> Vec<Vec<i32>> {
    let mut reports: Vec<Vec<i32>> = Vec::new();

    for line in read_to_string(FILE_PATH).unwrap().lines() {
        let new_line: Vec<i32> = line.split(" ").map(|x| x.parse::<i32>().unwrap()).collect();
        reports.push(new_line);
    }

    return reports;
}