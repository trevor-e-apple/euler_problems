use crate::problem_one::{
    problem_one_asm,
    problem_one_closed_form_plain,
    problem_one_plain
};

mod problem_one;

fn main() {
    println!("Problem one solution: {}", problem_one_plain(1000));
    println!("Problem one solution: {}", problem_one_closed_form_plain(1000));
    println!("Problem one solution: {}", problem_one_asm(1000));
}
