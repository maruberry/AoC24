use crate::days::dec1::dec1_1::get_input;

pub fn dec1_2() {
    let mut answer: u32 = 0;
    let (input1, input2) = get_input();
    for num in input1.iter() {
        answer += num * input2.clone().into_iter().filter(|n| n == num).count() as u32;
    }

    println!("The day 1 part 2 answer is {}", answer);
}