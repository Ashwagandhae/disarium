use rayon::prelude::*;

const NUM_DIGITS: usize = 22;
type Digit = u8;
type Number = u128;
#[derive(Debug, Clone)]
struct Digits<const NUM_DIGITS: usize> {
    digits: [Digit; NUM_DIGITS],
    first_non_zero_index: usize,
}

impl<const NUM_DIGITS: usize> Digits<NUM_DIGITS> {
    fn exp(&self) -> Number {
        calc_exp(&self.digits[self.first_non_zero_index..])
    }

    fn update_digit(&mut self, i: usize, new_digit: Digit) {
        self.digits[i] = new_digit;
        if i < self.first_non_zero_index && new_digit != 0 {
            self.first_non_zero_index = i;
        }
    }

    fn add_base_pow(&mut self, pow: usize) {
        for i in (0..self.digits.len()).rev().skip(pow) {
            if self.digits[i] < 9 {
                self.update_digit(i, self.digits[i] + 1);
                return;
            }
            self.update_digit(i, 0);
        }
    }
    fn from_number(number: Number) -> Self {
        let mut digits = [0; NUM_DIGITS];
        let mut n = number;
        let mut i = NUM_DIGITS;
        while n > 0 && i > 0 {
            i -= 1;
            digits[i] = (n % 10) as Digit;
            n /= 10;
        }
        let first_non_zero_index = i;

        Self {
            digits,
            first_non_zero_index,
        }
    }
    fn to_number(&self) -> Number {
        digits_to_num(&self.digits)
    }

    fn overwrite_digits(&mut self, digits: &[Digit]) {
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

    fn with_overwritten(mut self, digits: &[Digit]) -> Self {
        self.overwrite_digits(digits);
        self
    }

    fn min_for_digit_count(digit_count: u32) -> Self {
        let mut digits = [0; NUM_DIGITS];
        digits[digits.len() - digit_count as usize] = 1;
        Self {
            first_non_zero_index: digits.len() - digit_count as usize,
            digits,
        }
    }

    fn max_for_digit_count(digit_count: u32) -> Self {
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

fn num_digits(mut n: Number) -> u32 {
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
fn digits_to_num<const N: usize>(digits: &[Digit; N]) -> Number {
    let mut res: Number = 0;
    let mut base = 1;
    for d in digits[..].iter().rev() {
        res += (*d as Number) * base;
        base *= 10;
    }
    res
}

fn num_to_digits<const N: usize>(mut n: Number) -> [Digit; N] {
    let mut digits = [0; N];
    for i in (0..N).rev() {
        digits[i] = (n % 10) as Digit;
        n /= 10;
    }
    if n > 0 {
        panic!("Number too large to fit in {} digits", N);
    }
    digits
}

const DIGIT_POWERS: [[Number; NUM_DIGITS]; 10] = {
    let mut table = [[0; NUM_DIGITS]; 10];
    let mut d = 0;
    while d < 10 {
        let mut p = 0;
        while p < NUM_DIGITS {
            table[d][p] = (d as Number).pow((p + 1) as u32);
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

fn disarium_for_digit_count_with_frozen<const NUM_FROZEN: usize, const NUM_DIGITS: usize>(
    digit_count_unfrozen: u32,
    bound: Number,
    frozen_digits: [Digit; NUM_FROZEN],
) -> Vec<Number> {
    let digit_count = digit_count_unfrozen + NUM_FROZEN as u32;

    // println!(
    //     "for digit count: {} with frozen digits: {:?}",
    //     digit_count_unfrozen + NUM_FROZEN as u32,
    //     frozen_digits
    // );

    let min_digits: Digits<NUM_DIGITS> =
        Digits::min_for_digit_count(digit_count).with_overwritten(&frozen_digits);

    let max_digits: Digits<NUM_DIGITS> =
        Digits::max_for_digit_count(digit_count).with_overwritten(&frozen_digits);

    let start_digits: Digits<NUM_DIGITS> =
        Digits::from_number(min_digits.exp()).with_overwritten(&frozen_digits);
    let end_digits: Digits<NUM_DIGITS> =
        Digits::from_number(max_digits.exp()).with_overwritten(&frozen_digits);

    let (start_number, start_digits) = {
        let start_number = start_digits.to_number();
        let min_number = min_digits.to_number();
        if start_number >= min_number {
            (start_number, start_digits)
        } else {
            (min_number, min_digits)
        }
    };
    let end = end_digits
        .to_number()
        .min(max_digits.to_number())
        .min(bound);

    search_range::<NUM_FROZEN, NUM_DIGITS>(start_number, start_digits, end, NUM_FROZEN as u32)
}

fn search_range<const NUM_FROZEN: usize, const NUM_DIGITS: usize>(
    start_number: Number,
    start_digits: Digits<NUM_DIGITS>,
    end: Number,
    delta_pow: u32,
) -> Vec<Number> {
    let mut number: Number = start_number;
    let mut digits = start_digits;

    let mut res = Vec::new();

    let delta = (10 as Number).pow(delta_pow);

    while number <= end {
        if digits.exp() == number {
            res.push(number);
        }
        digits.add_base_pow(delta_pow as usize);
        number += delta;
    }
    return res;
}

fn disarium_for_digit_count<const NUM_FROZEN: usize, const NUM_DIGITS: usize>(
    digit_count: u32,
    bound: Number,
) -> Vec<Number> {
    let start_number = (10 as Number).pow(digit_count - 1);
    let start_digits = Digits::min_for_digit_count(digit_count);
    let end = ((10 as Number).pow(digit_count) - 1).min(bound);

    search_range::<NUM_FROZEN, NUM_DIGITS>(start_number, start_digits, end, 0)
}

fn freeze_and_split<const NUM_FROZEN: usize, const NUM_DIGITS: usize, const MULTI_THREAD: bool>(
    digit_count: u32,
    bound: Number,
) -> Vec<Number> {
    if digit_count <= NUM_FROZEN as u32 {
        disarium_for_digit_count::<NUM_FROZEN, NUM_DIGITS>(digit_count, bound)
    } else {
        let digit_count_unfrozen = digit_count - NUM_FROZEN as u32;
        if MULTI_THREAD {
            let mut res: Vec<_> = (0..(10 as Number))
                .into_par_iter()
                .flat_map(|i| {
                    (0..(10 as Number).pow(NUM_FROZEN as u32 - 1))
                        .flat_map(|frozen_number_lower| {
                            let frozen_number =
                                frozen_number_lower + i * (10 as Number).pow(NUM_FROZEN as u32 - 1);
                            let frozen_digits = num_to_digits::<NUM_FROZEN>(frozen_number);
                            disarium_for_digit_count_with_frozen::<NUM_FROZEN, NUM_DIGITS>(
                                digit_count_unfrozen,
                                bound,
                                frozen_digits,
                            )
                        })
                        .collect::<Vec<_>>()
                })
                .collect();
            res.sort();
            res
        } else {
            (0..(10 as Number).pow(NUM_FROZEN as u32))
                .flat_map(|frozen_number| {
                    let frozen_digits = num_to_digits::<NUM_FROZEN>(frozen_number);
                    disarium_for_digit_count_with_frozen::<NUM_FROZEN, NUM_DIGITS>(
                        digit_count_unfrozen,
                        bound,
                        frozen_digits,
                    )
                })
                .collect()
        }
    }
}

const FREEZE_AND_SPLIT_FUNCS: [fn(u32, Number) -> Vec<Number>; 26] = [
    freeze_and_split::<4, 4, false>,  // 0
    freeze_and_split::<4, 4, false>,  // 1
    freeze_and_split::<4, 4, false>,  // 2
    freeze_and_split::<4, 4, false>,  // 3
    freeze_and_split::<4, 4, false>,  // 4
    freeze_and_split::<2, 5, false>,  // 5
    freeze_and_split::<2, 6, false>,  // 6
    freeze_and_split::<2, 7, true>,   // 7
    freeze_and_split::<3, 8, true>,   // 8
    freeze_and_split::<3, 9, true>,   // 9
    freeze_and_split::<4, 10, true>,  // 10
    freeze_and_split::<4, 11, true>,  // 11
    freeze_and_split::<5, 12, true>,  // 12
    freeze_and_split::<5, 13, true>,  // 13
    freeze_and_split::<6, 14, true>,  // 14
    freeze_and_split::<6, 15, true>,  // 15
    freeze_and_split::<7, 16, true>,  // 16
    freeze_and_split::<7, 17, true>,  // 17
    freeze_and_split::<8, 18, true>,  // 18
    freeze_and_split::<8, 19, true>,  // 19
    freeze_and_split::<9, 20, true>,  // 20
    freeze_and_split::<9, 21, true>,  // 21
    freeze_and_split::<10, 22, true>, // 22
    freeze_and_split::<10, 23, true>, // 23
    freeze_and_split::<11, 24, true>, // 24
    freeze_and_split::<11, 25, true>, // 25
];

pub fn find_disarium(bound: Number) -> Vec<Number> {
    let max_digit_count = num_digits(bound);
    std::iter::once(0)
        .chain((1..=max_digit_count).flat_map(|digit_count| {
            FREEZE_AND_SPLIT_FUNCS[digit_count as usize](digit_count, bound)
        }))
        .collect()
}

pub fn find_disarium_for_digit_count(digit_count: u32) -> Vec<Number> {
    let max = (10 as Number).pow(digit_count) - 1;
    FREEZE_AND_SPLIT_FUNCS[digit_count as usize](digit_count, max)
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
