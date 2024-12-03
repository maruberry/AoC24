use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
fn get_input(report_str: &str) -> Vec<Vec<i32>> {
    let mut reports: Vec<Vec<i32>> = Vec::new();

    for line in report_str.lines() {
        let new_line: Vec<i32> = line.split_ascii_whitespace()
                            .map(|x| x.parse::<i32>()
                            .expect("This is not a number"))
                            .collect();
        reports.push(new_line);
    }

    return reports;
}

#[aoc(day2, part1)]
pub fn dec2_1(reports: &Vec<Vec<i32>>) -> usize{
    let answer = reports.iter().filter(|a| is_safe(a)).count();
    return answer;
}

#[aoc(day2, part2)]
pub fn dec2_2(reports: &Vec<Vec<i32>>) -> usize {
    let answer = reports.iter().filter(|a| is_safe(a) || is_safe_flaw(a)).count();
    return answer;
}

fn is_safe(report: &Vec<i32>) -> bool {
    let asc = report.is_sorted_by(|a, b| a < b && a+3 >= *b);
    let desc = report.is_sorted_by(|a, b| a > b && *a <= b+3);
    
    return asc || desc;
}

fn is_safe_flaw(report: &Vec<i32>) -> bool {
    for i in 0..report.len(){
        let mut new_report = report.clone();
        new_report.remove(i);
        if is_safe(&new_report) {
            return true;
        }
    }
    return false;
}