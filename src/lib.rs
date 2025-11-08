const NUM_DIGITS: usize = 20;
type Digit = u64;
#[derive(Debug, Clone)]
struct Digits {
    digits: [Digit; NUM_DIGITS],
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

    fn add_base_pow(&mut self, pow: usize) {
        for i in (0..self.digits.len()).rev().skip(pow) {
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
    fn to_number(&self) -> u64 {
        digits_to_num(&self.digits[self.first_non_zero_index..])
    }

    fn overwrite_digits(&mut self, digits: &[Digit]) {
        for (digit, replace_digit) in self.digits.iter_mut().rev().zip(digits.iter().rev()) {
            *digit = *replace_digit;
        }
    }

    fn with_overwritten(mut self, digits: &[Digit]) -> Self {
        self.overwrite_digits(digits);
        self
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
fn digits_to_num(digits: &[u64]) -> u64 {
    let mut res = 0;
    let mut base = 1;
    for d in digits[..].iter().rev() {
        res += d * base;
        base *= 10;
    }
    res
}

fn num_to_digits<const N: usize>(mut n: u64) -> [u64; N] {
    let mut digits = [0u64; N];
    for i in (0..N).rev() {
        digits[i] = n % 10;
        n /= 10;
    }
    if n > 0 {
        panic!("Number too large to fit in {} digits", N);
    }
    digits
}

const DIGIT_POWERS: [[u64; NUM_DIGITS]; 10] = {
    let mut table = [[0u64; NUM_DIGITS]; 10];
    let mut d = 0;
    while d < 10 {
        let mut p = 0;
        while p < NUM_DIGITS {
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

const NUM_FROZEN: usize = 2;

fn disarium_for_digit_count_with_frozen(
    digit_count_unfrozen: u32,
    bound: u64,
    frozen_digits: [Digit; NUM_FROZEN],
) -> Vec<u64> {
    let digit_count = digit_count_unfrozen + NUM_FROZEN as u32;

    // println!(
    //     "for digit count: {} with frozen digits: {:?}",
    //     digit_count_unfrozen + NUM_FROZEN as u32,
    //     frozen_digits
    // );

    let min_digits =
        Digits::from_number(10u64.pow(digit_count - 1)).with_overwritten(&frozen_digits);

    let max_digits =
        Digits::from_number(10u64.pow(digit_count) - 1).with_overwritten(&frozen_digits);

    let start_digits =
        Digits::from_number(min_digits.exp_by_index()).with_overwritten(&frozen_digits);
    let end_digits =
        Digits::from_number(max_digits.exp_by_index()).with_overwritten(&frozen_digits);

    let start = start_digits.to_number().max(min_digits.to_number());
    let end = end_digits
        .to_number()
        .min(max_digits.to_number())
        .min(bound);

    search_range(start, end, NUM_FROZEN as u32)
    // search_range(
    //     min_digits.to_number(),
    //     max_digits.to_number().min(bound),
    //     NUM_FROZEN as u32,
    // )
}

fn search_range(start: u64, end: u64, delta_pow: u32) -> Vec<u64> {
    let mut number: u64 = start;
    let mut digits = Digits::from_number(number);
    let mut res = Vec::new();

    let delta = 10u64.pow(delta_pow);

    while number <= end {
        if digits.exp_by_index() == number {
            res.push(number);
        }
        digits.add_base_pow(delta_pow as usize);
        number += delta;
    }
    return res;
}

fn disarium_for_digit_count(digit_count: u32, bound: u64) -> Vec<u64> {
    let start = 10u64.pow(digit_count - 1);
    let end = (10u64.pow(digit_count) - 1).min(bound);

    search_range(start, end, 0)
}

fn freeze_and_split(digit_count: u32, bound: u64) -> Vec<u64> {
    if digit_count <= NUM_FROZEN as u32 {
        disarium_for_digit_count(digit_count, bound)
    } else {
        let digit_count_unfrozen = digit_count - NUM_FROZEN as u32;
        (0..10u64.pow(NUM_FROZEN as u32))
            .flat_map(|frozen_number| {
                let frozen_digits = num_to_digits(frozen_number);
                disarium_for_digit_count_with_frozen(digit_count_unfrozen, bound, frozen_digits)
            })
            .collect()
    }
}

pub fn find_disarium(bound: u64) -> Vec<u64> {
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
