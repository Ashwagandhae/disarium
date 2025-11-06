const NUM_DIGITS: usize = 20;

#[derive(Debug)]
struct Digits {
    digits: [u64; NUM_DIGITS],
    first_non_zero_index: usize,
}

impl Digits {
    fn exp_by_index(&self) -> u64 {
        self.digits[self.first_non_zero_index..]
            .iter()
            .enumerate()
            .map(|(i, digit)| exp_digit(*digit as usize, i))
            .sum()
    }

    fn add_one(&mut self) {
        for i in (0..self.digits.len()).rev() {
            if self.digits[i] < 9 {
                self.digits[i] += 1;
                self.first_non_zero_index = self.first_non_zero_index.min(i);
                return;
            }
            self.digits[i] = 0;
        }
    }
    fn from_number(number: u64) -> Self {
        let mut digits = [0; NUM_DIGITS];
        let mut n = number;
        let mut i = NUM_DIGITS;
        while n > 0 && i > 0 {
            i -= 1;
            digits[i] = n % 10;
            n /= 10;
        }
        let first_non_zero_index = digits.iter().position(|&d| d != 0).unwrap_or(NUM_DIGITS);

        Self {
            digits,
            first_non_zero_index,
        }
    }
}

fn num_digits(mut n: u64) -> u32 {
    if n == 0 {
        return 0;
    }
    let mut count = 0;
    while n > 0 {
        count += 1;
        n /= 10;
    }
    count
}

const DIGIT_POWERS: [[u64; 20]; 10] = {
    let mut table = [[0u64; 20]; 10];
    let mut d = 0;
    while d < 10 {
        let mut p = 0;
        while p < 20 {
            table[d][p] = (d as u64).pow((p + 1) as u32);
            p += 1;
        }
        d += 1;
    }
    table
};
fn exp_digit(digit: usize, position: usize) -> u64 {
    DIGIT_POWERS[digit][position]
}

pub fn find_disarium_for_digit_count(digit_count: u32, bound: u64) -> Vec<u64> {
    let start = 10u64.pow(digit_count - 1);
    let end = (10u64.pow(digit_count) - 1).min(bound);

    let mut number: u64 = start;
    let mut digits = Digits::from_number(number);
    let mut res = Vec::with_capacity(20);

    while number <= end {
        if digits.exp_by_index() == number {
            res.push(number);
        }
        digits.add_one();
        number += 1;
    }
    return res;
}

pub fn find_disarium(bound: u64) -> Vec<u64> {
    let mut res = vec![0];
    let max_digit_count = num_digits(bound);
    for digit_count in 1..=max_digit_count {
        res.append(&mut find_disarium_for_digit_count(digit_count, bound));
    }
    res
}

#[cfg(test)]
mod tests {
    use crate::find_disarium;

    #[test]
    fn test_find_disarium() {
        assert_eq!(
            find_disarium(3_000_000),
            vec![
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 89, 135, 175, 518, 598, 1306, 1676, 2427, 2646798,
            ]
        );
    }
}
