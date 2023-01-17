use std::arch::asm;

fn triangle_number(n: i32) -> i32 {
    (n * (n + 1)) / 2
}

#[inline(never)]
pub fn problem_one_plain(n: i32) -> i32 {
    let mut sum = 0;
    for i in 0..n {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }

    sum
}

#[inline(never)]
pub fn problem_one_closed_form(n: i32) -> i32 {
    // the problem defines the sum of all numbers less than n
    let n = n - 1;

    let multiples_of_three = 3 * triangle_number(n / 3);
    let multiples_of_five = 5 * triangle_number(n / 5);
    let multiples_of_fifteen = 15 * triangle_number(n / 15);
    multiples_of_three + multiples_of_five - multiples_of_fifteen
}

#[cfg(target_arch="aarch64")]
fn sum_of_multiples_asm(n: u32, divisor: u32) -> u32 {
    let mut result: u32 = n;
    let divide_mul: u32 = divisor;
    unsafe {
        asm!(
            "udiv {result:x}, {result:x}, {divide_mul:x}",
            "mov {tmp}, {result:x}",
            "mul {result:x}, {result:x}, {result:x}",
            "add {result:x}, {result:x}, {tmp}",
            "asr {result:x}, {result:x}, 1",
            "mul {result:x}, {result:x}, {divide_mul:x}",
            divide_mul = in(reg) divide_mul,
            result = inout(reg) result,
            tmp = out(reg) _,
        );
    }
    result
}

#[cfg(target_arch="x86_64")]
fn sum_of_multiples_asm(n: u32, divisor: u32) -> u32 {
    let mut result: u32 = n;
    let divide_mul: u32 = divisor;
    unsafe {
        asm!(
            "mov eax, {result:e}",
            "mov ebx, {divide_mul:e}",
            "xor edx, edx", // clear edx register before div
            "div ebx", // n = result / divisor
            "mov ecx, eax", // store n in ecx
            "mul eax", // n ^ 2
            "add eax, ecx", // n ^ 2 + n
            "shr eax, 1", // (n ^ 2  + n) / 2
            "mul ebx", // eax = multiple * triangle_num
            "mov {result:e}, eax",
            divide_mul = in(reg) divide_mul,
            result = inout(reg) result
        );
    }

    result
}

#[inline(never)]
pub fn problem_one_asm(n: u32) -> u32 {
    let n = n - 1;

    let multiples_of_three = sum_of_multiples_asm(n, 3);
    let multiples_of_five = sum_of_multiples_asm(n, 5);
    let multiples_of_fifteen = sum_of_multiples_asm(n, 15);
    multiples_of_three + multiples_of_five - multiples_of_fifteen
}

#[bench]
fn plain(b: &mut test::Bencher) {
    b.iter(|| problem_one_plain(1000))
}

#[bench]
fn closed(b: &mut test::Bencher) {
    b.iter(|| problem_one_closed_form(1000))
}

#[bench]
fn asm(b: &mut test::Bencher) {
    b.iter(|| problem_one_asm(1000))
}