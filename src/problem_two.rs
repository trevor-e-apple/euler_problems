use std::arch::asm;

/// find the sum of all even fibonacci numbers less than 'a'
pub fn problem_two(a: i32) -> i32 {
    let mut n_1 = 1; // one back in the sequence
    let mut n_2 = 0; // two back in the sequence
    let mut n = n_1 + n_2;    
    let mut sum: i32 = 0;
    while n < a {
        n = n_1 + n_2;

        if n % 2 == 0 {
            sum += n;
        }

        n_2 = n_1;
        n_1 = n;
    }

    sum
}

#[cfg(target_arch="aarch64")]
pub fn problem_two_asm(a: i32) -> i32 {
    let n_1 = 1;
    let n_2 = 0;
    let n = n_1 + n_2;
    let mut sum: i32 = 0;
    unsafe {
        asm!(
            "2:", // loop start
            "cmp {n:x}, {a:x}", // loop condition (sum < a)
            "bge 4f", // go to loop end
            "add {n:x}, {n_1:x}, {n_2:x}",
            "and {tmp}, {n:x}, 1", // n % 2
            "cmp {tmp}, 0",
            "bne 3f", // go straight to updating recurrence
            "add {sum:x}, {sum:x}, {n:x}", // sum += n
            "3:", // update recurrence
            "mov {n_2:x}, {n_1:x}",
            "mov {n_1:x}, {n:x}",
            "b 2b", // go to loop start
            "4:", // loop end
            a = in(reg) a,
            n_1 = in(reg) n_1,
            n_2 = in(reg) n_2,
            n = in(reg) n,
            tmp = out(reg) _,
            sum = inout(reg) sum,
        );
    }
    sum
}

#[bench]
fn plain(b: &mut test::Bencher) {
    b.iter(|| problem_two(1_000_000_000))
}

#[bench]
fn asm(b: &mut test::Bencher) {
    b.iter(|| problem_two_asm(1_000_000_000))
}