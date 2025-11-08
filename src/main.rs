use disarium::find_disarium;

const BOUND: u128 = 10u128.pow(14);

fn main() {
    for num in find_disarium(BOUND.try_into().unwrap()) {
        println!("{num}");
    }
}
