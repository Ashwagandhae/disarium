use crate::{Digit, Number};

const DEC_DIGITS_LUT: [Digit; 200] = {
    let mut lut = [0 as Digit; 200];
    let mut i = 0;
    while i < 100 {
        lut[2 * i] = (i / 10) as Digit;
        lut[2 * i + 1] = (i % 10) as Digit;
        i += 1;
    }
    lut
};

pub fn int_to_digits_old(mut n: Number, buf: &mut [Digit]) -> usize {
    const DIGIT_PAIRS: [[Digit; 2]; 100] = {
        let mut pairs = [[0; 2]; 100];
        let mut i = 0;
        while i < 100 {
            pairs[i][0] = (i / 10) as Digit;
            pairs[i][1] = (i % 10) as Digit;
            i += 1;
        }
        pairs
    };

    let mut i = buf.len();

    while n >= 100 && i >= 2 {
        let pair = (n % 100) as usize;
        n /= 100;
        i -= 2;
        buf[i] = DIGIT_PAIRS[pair][0];
        buf[i + 1] = DIGIT_PAIRS[pair][1];
    }

    if n >= 10 {
        i -= 2;
        buf[i] = (n / 10) as Digit;
        buf[i + 1] = (n % 10) as Digit;
    } else if n > 0 {
        i -= 1;
        buf[i] = n as Digit;
    }
    i
}

#[inline]
pub fn int_to_digits(mut n: Number, buf: &mut [Digit]) -> usize {
    let lut = &DEC_DIGITS_LUT;

    let mut curr = buf.len();

    while n >= 10_000 {
        let rem = (n % 10_000) as usize;
        n /= 10_000;

        let d1 = (rem / 100) << 1;
        let d2 = (rem % 100) << 1;

        curr -= 4;
        buf[curr] = lut[d1];
        buf[curr + 1] = lut[d1 + 1];
        buf[curr + 2] = lut[d2];
        buf[curr + 3] = lut[d2 + 1];
    }

    if n >= 100 {
        let rem = n as usize;
        if rem >= 1000 {
            let d1 = (rem / 100) << 1;
            let d2 = (rem % 100) << 1;

            curr -= 4;
            buf[curr] = lut[d1];
            buf[curr + 1] = lut[d1 + 1];
            buf[curr + 2] = lut[d2];
            buf[curr + 3] = lut[d2 + 1];
        } else {
            // rem is 100..999
            let hundreds = rem / 100; // 1..9
            let last_two = rem % 100; // 0..99
            let d2 = last_two << 1;

            curr -= 3;
            buf[curr] = hundreds as Digit;
            buf[curr + 1] = lut[d2];
            buf[curr + 2] = lut[d2 + 1];
        }
    } else if n < 10 {
        curr -= 1;
        buf[curr] = n as Digit;
    } else {
        let d = (n as usize) << 1;

        curr -= 2;
        buf[curr] = lut[d];
        buf[curr + 1] = lut[d + 1];
    }
    curr
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reference_digits(mut n: Number) -> Vec<Digit> {
        let mut out = Vec::new();
        while n > 0 {
            out.push((n % 10) as Digit);
            n /= 10;
        }
        out.reverse();
        out
    }

    fn extract_digits(buf: &[Digit]) -> Vec<Digit> {
        let first_nonzero = buf.iter().position(|&d| d != 0).unwrap_or(buf.len());
        buf[first_nonzero..].to_vec()
    }

    #[test]
    fn test_small_numbers() {
        const N: usize = 20;
        let mut buf = [0 as Digit; N];

        for &n in &[0, 1, 5, 9, 10, 42, 99] {
            buf.fill(0);
            int_to_digits(n, &mut buf);
            let digits = extract_digits(&buf);
            assert_eq!(digits, reference_digits(n), "failed for n = {}", n);
        }
    }

    #[test]
    fn test_medium_numbers() {
        const N: usize = 20;
        let mut buf = [0 as Digit; N];

        for &n in &[123, 4567, 89012, 99999, 100000, 654321] {
            buf.fill(0);
            int_to_digits(n, &mut buf);
            let digits = extract_digits(&buf);
            assert_eq!(digits, reference_digits(n), "failed for n = {}", n);
        }
    }

    #[test]
    fn test_large_numbers() {
        const N: usize = 32;
        let mut buf = [0 as Digit; N];

        for &n in &[1_000_000_000, 4_294_967_295, 9_223_372] {
            buf.fill(0);
            int_to_digits(n, &mut buf);
            let digits = extract_digits(&buf);
            assert_eq!(digits, reference_digits(n), "failed for n = {}", n);
        }
    }
}
