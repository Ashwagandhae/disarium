use rayon::prelude::*;

use crate::digits::Digits;

pub mod digits;
pub mod itoa;

pub type Digit = u8;
pub type Number = u128;

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
    let min_number = (10 as Number).pow(digit_count - 1);

    let max_digits: Digits<NUM_DIGITS> =
        Digits::max_for_digit_count(digit_count).with_overwritten(&frozen_digits);
    let max_number = (10 as Number).pow(digit_count) - 1;

    let start_digits: Digits<NUM_DIGITS> =
        Digits::from_number_with_overwrite(min_digits.exp(), &frozen_digits);
    let end_digits: Digits<NUM_DIGITS> =
        Digits::from_number_with_overwrite(max_digits.exp(), &frozen_digits);

    let (start_number, start_digits) = {
        let start_number = start_digits.to_number();
        if start_number >= min_number {
            (start_number, start_digits)
        } else {
            (min_number, min_digits)
        }
    };
    let end = end_digits.to_number().min(max_number).min(bound);

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

    loop {
        if digits.exp() == number {
            res.push(number);
        }
        if number >= end {
            break;
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

fn freeze_and_split<
    const NUM_FROZEN: usize,
    const NUM_DIGITS: usize,
    const NUM_THREAD_DIGITS: u32,
>(
    digit_count: u32,
    bound: Number,
) -> Vec<Number> {
    if digit_count <= NUM_FROZEN as u32 {
        disarium_for_digit_count::<NUM_FROZEN, NUM_DIGITS>(digit_count, bound)
    } else {
        let digit_count_unfrozen = digit_count - NUM_FROZEN as u32;
        if NUM_THREAD_DIGITS > 0 {
            let mut res: Vec<_> = (0..(10 as Number).pow(NUM_THREAD_DIGITS))
                .into_par_iter()
                .flat_map(|i| {
                    (0..(10 as Number).pow(NUM_FROZEN as u32 - NUM_THREAD_DIGITS))
                        .flat_map(|frozen_number_lower| {
                            let frozen_number = frozen_number_lower
                                + i * (10 as Number).pow(NUM_FROZEN as u32 - NUM_THREAD_DIGITS);
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
    freeze_and_split::<4, 4, 0>,   // 0
    freeze_and_split::<4, 4, 0>,   // 1
    freeze_and_split::<4, 4, 0>,   // 2
    freeze_and_split::<4, 4, 0>,   // 3
    freeze_and_split::<4, 4, 0>,   // 4
    freeze_and_split::<2, 5, 0>,   // 5
    freeze_and_split::<2, 6, 0>,   // 6
    freeze_and_split::<2, 7, 1>,   // 7
    freeze_and_split::<3, 8, 1>,   // 8
    freeze_and_split::<3, 9, 1>,   // 9
    freeze_and_split::<4, 10, 2>,  // 10
    freeze_and_split::<4, 11, 2>,  // 11
    freeze_and_split::<5, 12, 2>,  // 12
    freeze_and_split::<5, 13, 2>,  // 13
    freeze_and_split::<6, 14, 2>,  // 14
    freeze_and_split::<6, 15, 2>,  // 15
    freeze_and_split::<7, 16, 2>,  // 16
    freeze_and_split::<7, 17, 2>,  // 17
    freeze_and_split::<8, 18, 2>,  // 18
    freeze_and_split::<8, 19, 2>,  // 19
    freeze_and_split::<9, 20, 3>,  // 20
    freeze_and_split::<9, 21, 3>,  // 21
    freeze_and_split::<10, 22, 3>, // 22
    freeze_and_split::<10, 23, 3>, // 23
    freeze_and_split::<11, 24, 3>, // 24
    freeze_and_split::<11, 25, 3>, // 25
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
