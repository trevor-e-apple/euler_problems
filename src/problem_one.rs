use std::arch::asm;

fn triangle_number(n: i32) -> i32 {
    (n * (n + 1)) / 2
}

pub fn problem_one_plain(n: i32) -> i32 {
    let mut sum = 0;
    for i in 0..n {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }

    sum
}

pub fn problem_one_closed_form_plain(n: i32) -> i32 {
    // the problem defines the sum of all numbers less than n
    let n = n - 1;

    let multiples_of_three = 3 * triangle_number(n / 3);
    let multiples_of_five = 5 * triangle_number(n / 5);
    let multiples_of_fifteen = 15 * triangle_number(n / 15);
    multiples_of_three + multiples_of_five - multiples_of_fifteen
}

pub fn triangle_number_asm(n: i32) -> i32 {
    let mut result: i32 = n;
    unsafe {
        asm!(
            "mov {tmp}, {result:x}",
            "mul {result:x}, {result:x}, {result:x}",
            "add {result:x}, {result:x}, {tmp}",
            "asr {result:x}, {result:x}, 1",
            result = inout(reg) result,
            tmp = out(reg) _,
        );
    }
    result
}

pub fn problem_one_asm(n: i32) -> i32 {
    let n = n - 1;

    let multiples_of_three = 3 * triangle_number_asm(n / 3);
    let multiples_of_five = 5 * triangle_number_asm(n / 5);
    let multiples_of_fifteen = 15 * triangle_number_asm(n / 15);
    multiples_of_three + multiples_of_five - multiples_of_fifteen
}
