#![feature(test)]
extern crate test;

use crate::problem_one::{
    problem_one_asm,
    problem_one_closed_form,
    problem_one_plain
};

mod problem_one;

fn main() {
    println!("Problem one solution: {}", problem_one_plain(1000));
    println!("Problem one solution: {}", problem_one_closed_form(1000));
    println!("Problem one solution: {}", problem_one_asm(1000));
}
