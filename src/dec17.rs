use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day17)]
fn get_input(input: &str) -> Computer {
    let (registers_long, program_long) = input.split_once("\n\n").unwrap();
    let regtemp = registers_long.split("\n").collect::<Vec<&str>>();
    let reg_vec: Vec<i128> = regtemp.iter()
        .map(|f| f[12..]
            .parse::<i128>()
            .unwrap()
        )
        .collect();

    let program: Vec<i128> = program_long[9..]
        .split(',')
        .map(|f| f
            .parse::<i128>()
            .unwrap()
        )
        .collect();

    let registers = Registers{a: reg_vec[0], b: reg_vec[1], c: reg_vec[2]};
    Computer{registers, program, index: 0, out: vec![]}
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Registers {
    a: i128,
    b: i128,
    c: i128
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Computer {
    registers: Registers,
    program: Vec<i128>,
    index: usize,
    out: Vec<i128>
}

impl Computer {
    fn combo_value(&self) -> i128{
        match self.program[self.index + 1] {
           0..=3 =>  self.program[self.index + 1],
           4 => self.registers.a,
           5 => self.registers.b,
           6 => self.registers.c,
           _ => panic!("invalid combo operand"),
        }
    }

    fn adv(&mut self) {
        //println!("adv aka 0");
        let combo_operand: u32 = self.combo_value() as u32;
        let pwr = 2i128.pow(combo_operand);
        if pwr == 0 {self.out.push(-4); return}
        let num = self.registers.a / pwr;
        self.registers.a = num;
        self.index += 2;
    }

    fn bxl(&mut self) {
        //println!("bxl aka 1");
        let num = self.registers.b as i128 ^ self.program[self.index + 1] as i128;
        self.registers.b = num as i128;
        self.index += 2;
    }

    fn bst(&mut self) {
        //println!("bst aka 2");
        let combo_operand: i128 = self.combo_value();
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
        let num = self.registers.b as i128 ^ self.registers.c as i128;
        self.registers.b = num as i128;
        self.index += 2;
    }

    fn out(&mut self) {
        //println!("out aka 5");
        let combo_operand: i128 = self.combo_value();
        self.out.push(combo_operand % 8);
        //println!("{}", combo_operand % 8);
        self.index += 2;
    }

    fn bdv(&mut self) {
        //println!("bdv aka 6");
        let combo_operand: u32 = self.combo_value() as u32;
        println!("combo {}", combo_operand);
        let pwr = 2i128.pow(combo_operand);
        if pwr == 0 {self.out.push(-4); return}
        let num = self.registers.a / pwr;
        self.registers.b = num;
        self.index += 2;
    }

    fn cdv(&mut self) {
        //println!("cdv aka 7");
        let combo_operand: u32 = self.combo_value() as u32;
        let pwr = 2i128.pow(combo_operand);
        if pwr == 0 {self.out.push(-4); return}
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
    //println!("{:?}", computer.program);
    while computer.index + 1 < computer.program.len(){
        computer.choose_program();
    }

    let out = computer.out;
    out.iter().join(",")
}


#[aoc(day17, part2)]
fn dec17_2(input: &Computer) -> i128{
    let mut computer = Computer {
        registers: input.registers.clone(),
        program: input.program.clone(),
        index: input.index.clone(),
        out: input.out.clone()
    };
    //println!("{:?}", computer.program);
    let mut visited: Vec<usize> = vec![];
    let mut counter = 0;
    let mut loops: u32 = 0;
    println!("WE ARE STARTING");
    while computer.index + 1 < computer.program.len(){
        //The program ends when A hits 0 (It will make no more jumps and thus will just... run to the end)
        //The value of A will also never be influenced by one of the registers even though it is TECHNICALLY possible
        //But it would make this task end far too quick
        //This is how we will find a minimum POSSIBLE value of A
        //From there I think it is reasonable to start just trying values
        //Could also assign a MAXIMUM possible value?
        if visited.contains(&computer.index) {
            println!("This loops after {} steps", counter);
            println!("In our loop we print out {} number(s)", computer.out.len());
            loops = (computer.program.len()/computer.out.len()) as u32;
            println!("We need to make {} loops", loops);
            break;
        }
        counter += 1;
        visited.push(computer.index);
        computer.choose_program();
    }
    //finding out where and how the value of our A is being influenced. A is being divided by either 2, 4 or 8
    //the operand of adv will never be 0 because 2^0 = 1 and then the value of a would just never change
    //the operand of adv will also never be larger than 3, because then it would use a register for the power 
    //and the loops would end REAAAAL quick (Or alternatively b and c will remain 0 forever)
    //if bdv or cdv ever happens is when values get very large and if they do not, it will remain 0
    let mut div = 1;
    for val in &visited {
        if computer.program[*val] == 0 {
            let num = 2i128.pow(computer.program[val + 1] as u32);
            div *= num;
        }
    }

    let mut min_a = div.pow(loops-1);
    let mut two = div.pow(1);


    computer.registers = input.registers.clone();
    computer.registers.a = min_a;
    computer.index = 0;
    computer.out = vec![];
    //println!("{:?}", computer.program);
    while computer.index + 1 < computer.program.len(){
        computer.choose_program();
    }

    let mut counter = 1;
    let mut prev_out = computer.out.clone();
    // loop {
    //     computer.registers = input.registers.clone();
    //     computer.registers.a = min_a + counter;
    //     computer.index = 0;
    //     computer.out = vec![];
    //     //println!("{:?}", computer.program);
    //     while computer.index + 1 < computer.program.len(){
    //         computer.choose_program();
    //     }
    //     if computer.out == computer.program{break;}
    //     counter += 1;
    //     //println!("a is {}", min_a + counter);
    // }

    println!("Trying to find something");
    //Find how often each index repeats
    for n in 0..computer.program.len() {
        let mut var = 0;
        let mut gap_a = vec![];
        let mut gap_b: Vec<i128> = vec![];
        let mut ahah = vec![];
        loop {
            computer.registers = input.registers.clone();
            computer.registers.a = min_a + var;
            computer.index = 0;
            computer.out = vec![];
            //println!("{:?}", computer.program);
            while computer.index + 1 < computer.program.len(){
                computer.choose_program();
            }
            if computer.out[n] == computer.program[n] {
                //println!("Found match for {n}");
                //println!("{var} and {gap_prev}");
                ahah.push(var);
                if gap_a.len() < 2 {
                    gap_a.push(var);
                }
                else {                
                    let num = ahah.len() - 2;
                    let newvar = var - ahah[num]; 
                    gap_a.push(newvar);
                    let l = gap_b.len();
                    if l == 0 && newvar == gap_a[1]{
                        gap_b.push(newvar);
                    }
                    else if gap_a[l+1] == newvar {
                        gap_b.push(newvar);
                        if gap_b == gap_a[1..l+1] && newvar == gap_b[0]{
                            break;
                        }
                    }
                    else if l != 0 {
                        gap_b = vec![];
                    }
                }
                //println!("A {:?}", gap_a);
                //println!("B {:?}", gap_b);
            }
            var += 1;
            //println!("a is {}", min_a + counter);
        }
        println!("for index {n} the gap is {:?}", gap_a);
    }

    println!("OUT {:?}", computer.out);
    println!("PRO {:?}", computer.program);

    //Finding out how often the first number changes


    min_a + counter
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_two() {
        let test_input: String = fs::read_to_string("input/2024/test_input/day17_2").unwrap();
        let input_filtered = get_input(&test_input);
        let ans = dec17_2(&input_filtered);
        assert_eq!(117440, ans);
    }

    #[test]
    fn test_one() {
        let test_input: String = fs::read_to_string("input/2024/test_input/day17").unwrap();
        let input_filtered = get_input(&test_input);
        let ans = dec17_1(&input_filtered);
        assert_eq!("4,6,3,5,6,3,5,2,1,0", ans);
    }
}