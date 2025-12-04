use aoc2025::by_line;

fn main() {
    run("./tests/day2");
    run("./input/day2");
}

fn valid(n: u64) -> (u64, u64) {
    let s = n.to_string();
    let str_len = s.len();

    let mut a = None;
    let mut b = None;

    'outer: for i in 1..=(str_len / 2) {
        let repeats = str_len / i;
        let rem = str_len % i;

        if rem != 0 {
            continue;
        }

        let pattern = &s[0..i];

        for j in 1..repeats {
            if s[(i * j)..(i * (j + 1))] != *pattern {
                continue 'outer;
            }
        }

        // println!("INVALID: {n}");
        if repeats == 2 && a.is_none() {
            a = Some(n);
        }
        if b.is_none() {
            b = Some(n);
        }
    }

    (*a.get_or_insert(0), *b.get_or_insert(0))
}

/// # Panics
///
/// Will panic if the file does contain other chars than ',', '-' or '0'..'9'
pub fn run(file: &str) {
    let mut sum_1 = 0;
    let mut sum_2 = 0;
    by_line(file, Some(b','), |chars| {
        let mut a: u64 = 0;
        let mut b: u64 = 0;

        let mut current = &mut a;

        for c in chars {
            match c {
                b'-' => current = &mut b,
                b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9' => {
                    let d: u64 = (c - b'0').into();
                    *current = 10 * *current + d;
                }
                _ => panic!("Unexpected char durring parsing: {c}"),
            }
        }

        for i in a..=b {
            let (a, b) = valid(i);
            sum_1 += a;
            sum_2 += b;
        }
    });

    println!("Part 1: {sum_1}");
    println!("Part 2: {sum_2}");
}
