use rayon::prelude::*;

const NUM_DIGITS: usize = 22;
type Digit = u8;
type Number = u128;
#[derive(Debug, Clone)]
struct Digits {
    digits: [Digit; NUM_DIGITS],
    first_non_zero_index: usize,
    exp: Number,
}

impl Digits {
    fn exp(&self) -> Number {
        self.exp
    }

    fn update_digit(&mut self, i: usize, new_digit: Digit) {
        if i < self.first_non_zero_index {
            self.digits[i] = new_digit;
            self.first_non_zero_index = i;
            self.exp = calc_exp(&self.digits[self.first_non_zero_index..]);
        } else {
            let position = i - self.first_non_zero_index;
            self.exp -= exp_digit(self.digits[i], position);
            self.digits[i] = new_digit;
            self.exp += exp_digit(self.digits[i], position);
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
        let first_non_zero_index = digits.iter().position(|&d| d != 0).unwrap_or(NUM_DIGITS);

        let exp = calc_exp(&digits[first_non_zero_index..]);

        Self {
            digits,
            first_non_zero_index,
            exp,
        }
    }
    fn to_number(&self) -> Number {
        digits_to_num(&self.digits[self.first_non_zero_index..])
    }

    fn overwrite_digits(&mut self, digits: &[Digit]) {
        for (i, replace_digit) in (0..self.digits.len()).rev().zip(digits.iter().rev()) {
            self.update_digit(i, *replace_digit);
        }
    }

    fn with_overwritten(mut self, digits: &[Digit]) -> Self {
        self.overwrite_digits(digits);
        self
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
fn digits_to_num(digits: &[Digit]) -> Number {
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
fn exp_digit(digit: Digit, position: usize) -> Number {
    DIGIT_POWERS[digit as usize][position]
}

const NUM_FROZEN: usize = 3;

fn disarium_for_digit_count_with_frozen(
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

    let min_digits =
        Digits::from_number((10 as Number).pow(digit_count - 1)).with_overwritten(&frozen_digits);

    let max_digits =
        Digits::from_number((10 as Number).pow(digit_count) - 1).with_overwritten(&frozen_digits);

    let start_digits = Digits::from_number(min_digits.exp()).with_overwritten(&frozen_digits);
    let end_digits = Digits::from_number(max_digits.exp()).with_overwritten(&frozen_digits);

    let start = start_digits.to_number().max(min_digits.to_number());
    let end = end_digits
        .to_number()
        .min(max_digits.to_number())
        .min(bound);

    search_range(start, end, NUM_FROZEN as u32)
}

fn search_range(start: Number, end: Number, delta_pow: u32) -> Vec<Number> {
    let mut number: Number = start;
    let mut digits = Digits::from_number(number);
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

fn disarium_for_digit_count(digit_count: u32, bound: Number) -> Vec<Number> {
    let start = (10 as Number).pow(digit_count - 1);
    let end = ((10 as Number).pow(digit_count) - 1).min(bound);

    search_range(start, end, 0)
}

fn freeze_and_split(digit_count: u32, bound: Number) -> Vec<Number> {
    if digit_count <= NUM_FROZEN as u32 {
        disarium_for_digit_count(digit_count, bound)
    } else {
        let digit_count_unfrozen = digit_count - NUM_FROZEN as u32;
        // (0..(10 as Number).pow(NUM_FROZEN as u32))
        //     .flat_map(|frozen_number| {
        //         let frozen_digits = num_to_digits(frozen_number);
        //         disarium_for_digit_count_with_frozen(digit_count_unfrozen, bound, frozen_digits)
        //     })
        //     .collect()
        let mut res: Vec<_> = (0..(10 as Number))
            .into_par_iter()
            .flat_map(|i| {
                (0..(10 as Number).pow(NUM_FROZEN as u32 - 1))
                    .flat_map(|frozen_number_lower| {
                        let frozen_number =
                            frozen_number_lower + i * (10 as Number).pow(NUM_FROZEN as u32 - 1);
                        let frozen_digits = num_to_digits(frozen_number);
                        disarium_for_digit_count_with_frozen(
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
    }
}

pub fn find_disarium(bound: Number) -> Vec<Number> {
    let max_digit_count = num_digits(bound);
    std::iter::once(0)
        .chain((1..=max_digit_count).flat_map(|digit_count| freeze_and_split(digit_count, bound)))
        .collect()
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
