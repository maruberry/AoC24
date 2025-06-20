use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day17)]
fn get_input(input: &str) -> Computer {
    let (registers_long, program_long) = input.split_once("\n\n").unwrap();
    let regtemp = registers_long.split("\n").collect::<Vec<&str>>();
    let reg_vec: Vec<u128> = regtemp.iter()
        .map(|f| f[12..]
            .parse::<u128>()
            .unwrap()
        )
        .collect();

    let program: Vec<u128> = program_long[9..]
        .split(',')
        .map(|f| f
            .parse::<u128>()
            .unwrap()
        )
        .collect();

    let registers = Registers{a: reg_vec[0], b: reg_vec[1], c: reg_vec[2]};
    Computer{registers, program, index: 0, out: vec![]}
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Registers {
    a: u128,
    b: u128,
    c: u128
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Computer {
    registers: Registers,
    program: Vec<u128>,
    index: usize,
    out: Vec<u128>
}

impl Computer {
    fn combo_value(&self, literal: u128) -> u128{
        match literal {
           0..=3 =>  literal,
           4 => self.registers.a,
           5 => self.registers.b,
           6 => self.registers.c,
           _ => panic!("invalid combo operand"),
        }
    }

    fn adv(&mut self) {
        //println!("adv aka 0");
        let combo_operand: u32 = self.combo_value(self.program[self.index + 1]) as u32;
        let pwr = 2u128.pow(combo_operand);
        //if pwr == 0 {self.out.push(-4); return}
        let num = self.registers.a / pwr;
        self.registers.a = num;
        self.index += 2;
    }

    fn bxl(&mut self) {
        //println!("bxl aka 1");
        let num = self.registers.b as u128 ^ self.program[self.index + 1] as u128;
        self.registers.b = num as u128;
        self.index += 2;
    }

    fn bst(&mut self) {
        //println!("bst aka 2");
        let combo_operand: u128 = self.combo_value(self.program[self.index + 1]);
        let num = combo_operand % 8;
        self.registers.b = num;
        self.index += 2;
    }

    fn jnz(&mut self) {
        //println!("jnz aka 3");
        if self.registers.a == 0 {self.index += 2; return}
        self.index = self.program[self.index + 1] as usize;
    }

    fn bxc(&mut self) {
        //println!("bxc aka 4") ;
        let num = self.registers.b as u128 ^ self.registers.c as u128;
        self.registers.b = num as u128;
        self.index += 2;
    }

    fn out(&mut self) {
        //println!("out aka 5");
        let combo_operand: u128 = self.combo_value(self.program[self.index + 1]);
        self.out.push(combo_operand % 8);
        //println!("{}", combo_operand % 8);
        self.index += 2;
    }

    fn bdv(&mut self) {
        //println!("bdv aka 6");
        let combo_operand: u32 = self.combo_value(self.program[self.index + 1]) as u32;
        println!("combo {}", combo_operand);
        let pwr = 2u128.pow(combo_operand);
        //if pwr == 0 {self.out.push(-4); return}
        let num = self.registers.a / pwr;
        self.registers.b = num;
        self.index += 2;
    }

    fn cdv(&mut self) {
        //println!("cdv aka 7");
        let combo_operand: u32 = self.combo_value(self.program[self.index + 1]) as u32;
        let pwr = 2u128.pow(combo_operand);
        //if pwr == 0 {self.out.push(-4); return}
        let num = self.registers.a / pwr;
        self.registers.c = num;
        self.index += 2;
    }

    fn choose_program(&mut self) {
        match self.program[self.index] {
            0 => self.adv(),
            1 => self.bxl(),
            2 => self.bst(),
            3 => self.jnz(),
            4 => self.bxc(),
            5 => self.out(),
            6 => self.bdv(),
            7 => self.cdv(),
            _ => panic!("Invalid program number")
        }
    }
}


#[aoc(day17, part1)]
fn dec17_1(input: &Computer) -> String{
    let mut computer = Computer {
        registers: input.registers.clone(),
        program: input.program.clone(),
        index: input.index.clone(),
        out: input.out.clone()
    };
    while computer.index + 1 < computer.program.len(){
        computer.choose_program();
    }
    let out = computer.out;
    out.iter().join(",")
}


#[aoc(day17, part2)]
fn dec17_2(input: &Computer) -> u128{
    let mut computer = Computer {
        registers: input.registers.clone(),
        program: input.program.clone(),
        index: input.index.clone(),
        out: input.out.clone()
    };
    let mut visited: Vec<usize> = vec![];
    let mut loops: u32 = 0;
    let mut prev = 0;
    let mut index = 0;
    let mut operand = 0;
    computer.registers.a = 35184372088832;
    let mut counter = 0;
    while computer.index + 1 < computer.program.len(){

        if visited.contains(&computer.index){
            loops = (computer.program.len()/computer.out.len()) as u32;
            println!("We need to make {} loops", loops);
            break;
        }

        visited.push(computer.index);
        computer.choose_program();

        if prev != computer.registers.a {
            computer.registers.a = computer.registers.a;
            prev = computer.registers.a;
            operand = counter;
            println!("Operand is {}", computer.program[index + 1]);
        }
        counter += 1;
        //println!("A: {}", computer.registers.a);
    }
    //We know that A can only ever be changed by register adv. It will NEVER use values in
    //any other register bcs then the solution would be too small.

    let mut min_a: u128 = 8i128.pow(loops - 1) as u128;
    println!("Min A is {}", min_a);
    let mut exps: Vec<u32> = Vec::new();
    for i in (0..input.program.len()).rev(){
        let exp = find_exp(min_a, input, i);
        let num = find_first_occurrance(min_a, input, i, exp);
        let min = find_smallest_for_index(num, input, i, exp);
        println!("We found at {}", min);
        min_a = min;
        //println!("The minimum for index {} is {}", i, min);
        //min_a = min;
        //156985291375904 w num
        //156985291375904 w min
        let mut computer = Computer {
            registers: input.registers.clone(),
            program: input.program.clone(),
            index: input.index.clone(),
            out: input.out.clone()
        };
        computer.registers.a = min_a;

        while computer.index + 1 < computer.program.len(){
            computer.choose_program();
        }

        println!("\nOUTPUT {:?}\n", computer.out);
    }

    // let mut temp_a = min_a;
    // while computer.out != input.program{
    //     let mut computer = Computer {
    //         registers: input.registers.clone(),
    //         program: input.program.clone(),
    //         index: input.index.clone(),
    //         out: input.out.clone()
    //     };
    //     computer.registers.a = temp_a + 1;
    //     while computer.index + 1 < computer.program.len(){
    //         computer.choose_program();
    //     }
    // }

    min_a
}

fn find_first_occurrance(min_a: u128, input: &Computer, index: usize, exp: u32) -> u128 {
    let step = 4u128.pow(exp);
    let mut temp = min_a;

    loop {
        let mut computer = Computer {
            registers: input.registers.clone(),
            program: input.program.clone(),
            index: input.index.clone(),
            out: input.out.clone()
        };
        computer.registers.a = temp;

        while computer.index + 1 < computer.program.len(){
            computer.choose_program();
        }
        if input.program[index] == computer.out[index] {
            return temp;
        }

        temp = temp + step;
    }
}
fn find_smallest_for_index(min_a: u128, input: &Computer, index: usize, mut exp: u32) -> u128 {
    let mut prev_output: Vec<u128> = vec![];
    exp -= 1;
    let mut step = 4u128.pow(exp);
    let mut temp = min_a - step;
   loop {
        'inner: for _i in 0..16{
            let mut computer = Computer {
                registers: input.registers.clone(),
                program: input.program.clone(),
                index: input.index.clone(),
                out: input.out.clone()
            };
            computer.registers.a = temp;

            while computer.index + 1 < computer.program.len(){
                computer.choose_program();
            }
            //println!("{:?}", computer.out);

            if input.program[index] != computer.out[index] {
                temp = temp + step;
                break 'inner;
            }
            temp = temp - step;
        }

        if step == 1 {return temp;}

        step = step / 2;
        temp = temp - step;
    }
}

fn find_exp(min_a: u128, input: &Computer, index: usize) -> u32{
    let mut prev_output: Vec<u128> = vec![];
    let mut step = 4u128.pow(1);
    let mut exp = 1;
    let mut temp = min_a;
    //58640620148048
    loop {
        for _i in 0..16{
            let mut computer = Computer {
                registers: input.registers.clone(),
                program: input.program.clone(),
                index: input.index.clone(),
                out: input.out.clone()
            };
            computer.registers.a = temp;

            while computer.index + 1 < computer.program.len(){
                computer.choose_program();
            }
            //println!("{:?}", computer.out);
            if prev_output.len() != 0 {
                if prev_output[index] != computer.out[index] {
                    println!("index {} took exp {} and val is {}", index, exp, temp);
                    return exp;
                }
            }
            prev_output = computer.out;
            temp = temp + step;
        }

        exp += 1;
        step = 4u128.pow(exp);
        temp = min_a + step;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_two() {
        let test_input: String = fs::read_to_string("input/2024/day17_test2.txt").unwrap();
        let input_filtered = get_input(&test_input);
        let ans = dec17_2(&input_filtered);
        assert_eq!(117440, ans);
    }

    #[test]
    fn test_one() {
        let test_input: String = fs::read_to_string("input/2024/day17_test1.txt").unwrap();
        let input_filtered = get_input(&test_input);
        let ans = dec17_1(&input_filtered);
        assert_eq!("4,6,3,5,6,3,5,2,1,0", ans);
    }
}