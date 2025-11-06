fn add_one(digits: &mut [u8], number: &mut u64) {
    *number += 1;
    for i in (0..digits.len()).rev() {
        if digits[i] < 9 {
            digits[i] += 1;
            return;
        }
        digits[i] = 0;
    }
}

fn check_disarium(digits: &[u8], number: u64) -> bool {
    let number_cmp: u64 = digits
        .iter()
        .skip_while(|&n| *n == 0)
        .enumerate()
        .map(|(i, n)| (*n as u64).pow(i as u32 + 1))
        .sum();
    number_cmp == number
}

pub fn find_disarium(bound: u64) -> impl Iterator<Item = u64> {
    let mut digits = [0; 10];
    let mut number: u64 = 0;
    std::iter::from_fn(move || {
        while number < bound {
            add_one(&mut digits, &mut number);
            if check_disarium(&digits, number) {
                return Some(number);
            }
        }
        None
    })
}
