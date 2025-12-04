use aoc2025::by_line;

fn main() {
    run("./tests/day1");
    run("./input/day1");
}

#[derive(Debug)]
struct Dial {
    value: i32,
}

impl Default for Dial {
    fn default() -> Self {
        Self::new()
    }
}

impl Dial {
    #[must_use]
    const fn new() -> Self {
        Self { value: 50 }
    }

    #[must_use]
    const fn left(&mut self, value: i32) -> (i32, i32) {
        let mut erg = if self.value == 0 { -1 } else { 0 };
        self.value -= value;
        while self.value < 0 {
            self.value += 100;
            erg += 1;
        }
        let at_zero = if self.value == 0 { 1 } else { 0 };
        (at_zero, at_zero + erg)
    }

    #[must_use]
    const fn right(&mut self, value: i32) -> (i32, i32) {
        self.value += value;
        let mut erg = 0;
        while self.value > 99 {
            self.value -= 100;
            erg += 1;
        }
        let at_zero = if self.value == 0 { 1 } else { 0 };
        (at_zero, erg)
    }
}

pub fn run(file: &str) {
    let mut dial = Dial::new();
    let mut erg_1 = 0;
    let mut erg_2 = 0;

    by_line(file, |line| {
        let chars: Vec<_> = line.bytes().collect();
        let left = chars[0] == b'L';
        let mut count: i32 = (chars[1] - b'0').into();
        if let Some(c) = chars.get(2) {
            let d: i32 = (c - b'0').into();
            count = count * 10 + d;
            if let Some(c) = chars.get(3) {
                let d: i32 = (c - b'0').into();
                count = count * 10 + d;
            }
        }

        let (c_1, c_2) = if left {
            dial.left(count)
        } else {
            dial.right(count)
        };

        erg_1 += c_1;
        erg_2 += c_2;
    });
    println!("{erg_1}");
    println!("{erg_2}");
}
