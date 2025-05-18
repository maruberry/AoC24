use aoc_runner_derive::{aoc, aoc_generator};
use ndarray::array;
use ndarray_linalg::Solve;

#[aoc_generator(day13)]
fn get_input(input: &str) -> (Vec<(f64, f64)>, Vec<(f64, f64)>, Vec<(f64, f64)>){ 
    let mut button_a: Vec<(f64, f64)> = vec![];
    let mut button_b: Vec<(f64, f64)> = vec![];
    let mut prize: Vec<(f64, f64)> = vec![];
    
    for line in input.lines() {
        if line.contains("A:") {
            let nline = &line.replace("Button A: X+", "");
            let nline = &nline.replace(" Y+", "");
            let xy: Vec<&str> = nline.split(',').collect();
            button_a.push((xy[0].parse().unwrap(), xy[1].parse().unwrap()));
        }
        else if line.contains("Prize") {
            let nline = &line.replace("Prize: X=", "");
            let nline = &nline.replace(" Y=", "");
            let xy: Vec<&str> = nline.split(',').collect();
            prize.push((xy[0].parse().unwrap(), xy[1].parse().unwrap()));
        }
        else if line.contains("B:") {
            let nline = &line.replace("Button B: X+", "");
            let nline = &nline.replace(" Y+", "");
            let xy: Vec<&str> = nline.split(',').collect();
            button_b.push((xy[0].parse().unwrap(), xy[1].parse().unwrap()));
        }
    }
    
    return (button_a, button_b, prize)
}

#[aoc(day13, part1)]
fn dec12_1(input: &(Vec<(f64, f64)>, Vec<(f64, f64)>, Vec<(f64, f64)>)) -> f64{

    let (button_a, button_b, prize) = input;
    let calc = button_a.iter().zip(button_b).zip(prize);
    let mut final_ans: f64 = 0f64;
    for ((a, b), prize) in calc {
        let matr = array![[a.0, b.0], [a.1, b.1]];
        let ans = array![prize.0, prize.1];
        let x = matr.solve(&ans);
        final_ans += match x {
            Ok(lmao) => {
                if lmao[0] >= 0f64 && lmao[1] >= 0f64 {
                    let mut a = 0f64;
                    let mut b = 0f64;
                    if lmao[0].fract() < 0.0000000001 {
                        a = lmao[0].trunc( );
                    }
                    if lmao[0].fract() > 0.9999999999 {
                        a = lmao[0].ceil();
                    }
                    if lmao[1].fract() < 0.0000000001 {
                        b = lmao[1].trunc( );
                    }
                    if lmao[1].fract() > 0.9999999999 {
                        b = lmao[1].ceil();
                    }
                    //println!("{} and {}", a, b);
                    a * 3f64 + b
                }
                else {
                    0f64
                }
            },
            Err(_) => 0f64,
        }
    }
    final_ans
}


#[aoc(day13, part2)]
fn dec12_2(input: &(Vec<(f64, f64)>, Vec<(f64, f64)>, Vec<(f64, f64)>)) -> f64{

    let (button_a, button_b, prize) = input;
    let calc = button_a.iter().zip(button_b).zip(prize);
    let mut final_ans: f64 = 0f64;
    for ((a, b), prize) in calc {
        let matr = array![[a.0, b.0], [a.1, b.1]];
        let ans = array![prize.0 + 10000000000000f64, prize.1 + 10000000000000f64];
        let x = matr.solve(&ans);
        final_ans += match x {
            Ok(lmao) => {
                if lmao[0] >= 0f64 && lmao[1] >= 0f64{
                    let mut a = 0f64;
                    let mut b = 0f64;
                    if lmao[0].fract() < 0.0001 {
                        a = lmao[0].trunc( );
                    }  
                    if lmao[0].fract() > 0.9999 {
                        a = lmao[0].ceil();
                    }
                    if lmao[1].fract() < 0.0001 {
                        b = lmao[1].trunc( );
                    }
                    if lmao[1].fract() > 0.9999 {
                        b = lmao[1].ceil();
                    }
                    //println!("{} and {}", a, b);
                    if a != 0.0 && b != 0.0 {
                        a * 3f64 + b
                    }
                    else {
                        0.0
                    }
                }
                else {
                    0f64
                }
            },
            Err(_) => 0f64,
        }
    }
    final_ans
}

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input_filtered = get_input(SMALL_INPUT);
        let ans = dec12_1(&input_filtered);
        assert_eq!(480f64, ans);
    }

    #[test]
    fn test_part2() {
        let input_filtered = get_input(SMALL_INPUT);
        let ans = dec12_2(&input_filtered);
        assert_eq!(480f64, ans);
    }

    const SMALL_INPUT:&str = 
"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"; 
}