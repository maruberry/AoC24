use crate::days::dec1::dec1_1::dec1_1;
use crate::days::dec1::dec1_2::dec1_2;
use crate::days::dec2::dec2_1::dec2_1;
use crate::days::dec2::dec2_2::dec2_2;
pub mod days;

static DAY: usize = 2;
static PART: usize = 2;

fn main() {
    match DAY {
        1 => {
            match PART {
                1 => dec1_1(),
                2 => dec1_2(),
                _ => println!("invalid part"),
            }
        },
        2 => {
            match PART {
                1 => dec2_1(),
                2 => dec2_2(),
                _ => println!("invalid part"),
            }
        },
        _ => println!("invalid day"),
    }
}
