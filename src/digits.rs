use crate::{Digit, Number, itoa::int_to_digits};
const MAX_DIGITS: usize = 20;

#[derive(Debug, Clone)]
pub struct Digits<const NUM_DIGITS: usize> {
    digits: [Digit; NUM_DIGITS],
    first_non_zero_index: usize,
}

impl<const NUM_DIGITS: usize> Digits<NUM_DIGITS> {
    pub fn exp(&self) -> Number {
        calc_exp(&self.digits[self.first_non_zero_index..])
    }

    fn update_digit(&mut self, i: usize, new_digit: Digit) {
        self.digits[i] = new_digit;
        if i < self.first_non_zero_index && new_digit != 0 {
            self.first_non_zero_index = i;
        }
    }

    pub fn add_base_pow(&mut self, pow: usize) {
        for i in (0..self.digits.len()).rev().skip(pow) {
            if self.digits[i] < 9 {
                self.update_digit(i, self.digits[i] + 1);
                return;
            }
            self.update_digit(i, 0);
        }
    }
    pub fn from_number(n: Number) -> Self {
        let mut digits = [0; NUM_DIGITS];
        let first_non_zero_index = int_to_digits(n, &mut digits);

        Self {
            digits,
            first_non_zero_index,
        }
    }

    pub fn from_number_with_overwrite<const N: usize>(
        n: Number,
        overwrite_digits: &[Digit; N],
    ) -> Self {
        let mut digits = [(0 as Digit); NUM_DIGITS];
        let overwrite_start = digits.len() - overwrite_digits.len();
        digits[overwrite_start..].copy_from_slice(overwrite_digits);
        let index = int_to_digits(
            n / (10 as Number).pow(N as u32),
            &mut digits[..overwrite_start],
        );

        let first_non_zero_index = if index == overwrite_start {
            overwrite_start
                + digits[overwrite_start..]
                    .iter()
                    .position(|&d| d != 0)
                    .unwrap_or(overwrite_digits.len())
        } else {
            index
        };

        Self {
            digits,
            first_non_zero_index,
        }
    }

    pub fn to_number(&self) -> Number {
        digits_to_num(&self.digits)
    }

    fn overwrite_digits<const N: usize>(&mut self, digits: &[Digit; N]) {
        let start = self.digits.len() - digits.len();
        self.digits[start..].copy_from_slice(digits);
        if start < self.first_non_zero_index {
            self.first_non_zero_index = start
                + self.digits[start..]
                    .iter()
                    .position(|&d| d != 0)
                    .unwrap_or(self.digits[start..].len());
        }
    }

    pub fn with_overwritten<const N: usize>(mut self, digits: &[Digit; N]) -> Self {
        self.overwrite_digits::<N>(digits);
        self
    }

    pub fn min_for_digit_count(digit_count: u32) -> Self {
        let mut digits = [0; NUM_DIGITS];
        digits[digits.len() - digit_count as usize] = 1;
        Self {
            first_non_zero_index: digits.len() - digit_count as usize,
            digits,
        }
    }

    pub fn max_for_digit_count(digit_count: u32) -> Self {
        let mut digits = [0; NUM_DIGITS];
        for i in (0..NUM_DIGITS).rev().take(digit_count as usize) {
            digits[i] = 9;
        }
        Self {
            first_non_zero_index: digits.len() - digit_count as usize,
            digits,
        }
    }
}

fn calc_exp(digits: &[Digit]) -> Number {
    digits
        .iter()
        .enumerate()
        .map(|(position, digit)| exp_digit(*digit, position))
        .sum()
}

const DIGIT_POWERS: [[Number; MAX_DIGITS]; 10] = {
    let mut table = [[0; MAX_DIGITS]; 10];
    let mut d = 0;
    while d < 10 {
        let mut p = 0;
        while p < MAX_DIGITS {
            table[d][p] = (d as Number).saturating_pow((p + 1) as u32);
            p += 1;
        }
        d += 1;
    }
    table
};

#[inline]
fn exp_digit(digit: Digit, position: usize) -> Number {
    DIGIT_POWERS[digit as usize][position]
}

fn digits_to_num<const N: usize>(digits: &[Digit; N]) -> Number {
    let mut res: Number = 0;
    let mut base = 1;
    for d in digits[..].iter().rev() {
        let res_add = (*d as Number).saturating_mul(base);

        res = res.saturating_add(res_add);
        base = base.saturating_mul(10);
    }
    res
}
