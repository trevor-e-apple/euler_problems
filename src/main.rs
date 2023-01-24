#![feature(test)]
extern crate test;

use crate::problem_one::{
    problem_one_asm,
    problem_one_closed_form,
    problem_one_plain,
};
use crate::problem_two::{problem_two, problem_two_asm};

mod problem_one;
mod problem_two;

fn main() {
    println!("Problem one solution: {}", problem_one_plain(1000));
    println!("Problem one solution: {}", problem_one_closed_form(1000));
    println!("Problem one solution: {}", problem_one_asm(1000));

    println!("Problem two solution: {}", problem_two(4_000_000));
    println!("Problem two solution: {}", problem_two_asm(4_000_000));
}
