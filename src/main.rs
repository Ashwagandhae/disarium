use disarium::find_disarium;

const BOUND: u128 = 10u128.pow(10);

fn main() {
    for num in find_disarium(BOUND.try_into().unwrap()) {
        println!("{num}");
    }
}
