use crate::days::dec2::dec2_1::read_input;

pub fn dec2_2() {
    let mut answer = 0;
    let reports = read_input();

    for report in reports {
        if is_ascdesc(report.clone()){
            answer += 1;
        }
        else {
            for i in 0..report.len(){
                let mut new_report = report.clone();
                new_report.remove(i);
                println!("{:?}", nReport);
                if is_ascdesc(new_report.clone()) {
                    answer += 1;
                    break;
                }
            }
        }
    }

    println!("Day 2 part 2 answer is: {}", answer);
}

fn is_ascdesc(report: Vec<i32>) -> bool {
    let asc = report.is_sorted_by(|a, b| a < b && a+3 >= *b);
    let desc = report.is_sorted_by(|a, b| a > b && *a <= b+3);
    
    return asc || desc;
}