use disarium::find_disarium;

const BOUND: u64 = 10_000_000_000;

fn main() {
    for num in find_disarium(BOUND.try_into().unwrap()) {
        println!("{num}");
    }
}
