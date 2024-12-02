use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
fn get_input(report_str: &str) -> Vec<Vec<i32>> {
    let mut reports: Vec<Vec<i32>> = Vec::new();

    for line in report_str.lines() {
        let new_line: Vec<i32> = line.split_ascii_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
        reports.push(new_line);
    }

    return reports;
}

#[aoc(day2, part1)]
pub fn dec2_1(reports: &Vec<Vec<i32>>) -> i32{
    let mut answer = 0;
    for report in reports {
        if is_ascdesc(report.clone()) {
            answer += 1;
        }
    }
    return answer;
}

#[aoc(day2, part2)]
pub fn dec2_2(reports: &Vec<Vec<i32>>) -> i32 {
    let mut answer = 0;

    'outer: for report in reports {
        if is_ascdesc(report.clone()){
            answer += 1;
            continue;
        }
        for i in 0..report.len(){
            let mut new_report = report.clone();
            new_report.remove(i);
            if is_ascdesc(new_report.clone()) {
                answer += 1;
                continue 'outer;
            }
        }
    }

    return answer;
}

fn is_ascdesc(report: Vec<i32>) -> bool {
    let asc = report.is_sorted_by(|a, b| a < b && a+3 >= *b);
    let desc = report.is_sorted_by(|a, b| a > b && *a <= b+3);
    
    return asc || desc;
}