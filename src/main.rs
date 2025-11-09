use std::time::Instant;

use disarium::find_disarium;

const BOUND: u128 = 10u128.pow(20);

fn main() {
    let start = Instant::now();
    for num in find_disarium(BOUND.try_into().unwrap()) {
        println!("{num}");
    }
    let duration = start.elapsed();

    println!(
        "found numbers in {} ms, or {} secs",
        duration.as_millis(),
        duration.as_secs()
    );
}

// fn time_digit_count(digit_count: u32) {
//     let max = (10 as u128).pow(digit_count) - 1;
//     println!("warming up...");
//     let start = Instant::now();
//     for _ in 0..1000 {
//         FREEZE_AND_SPLIT_VARIANTS[5](digit_count, max);
//     }
//     let duration = start.elapsed();
//     println!("warm up for: {}", duration.as_nanos());
//     println!("testing digit count: {}", digit_count);
//     for i in (0..FREEZE_AND_SPLIT_VARIANTS.len()) {
//         let variant = FREEZE_AND_SPLIT_VARIANTS[i];
//         let start = Instant::now();
//         for _ in 0..100 {
//             variant(digit_count, max);
//         }
//         let duration = start.elapsed();
//         println!(
//             "duration of variant {i} (number {}): {} nanosecs",
//             i + 2,
//             duration.as_nanos()
//         );
//     }
// }
