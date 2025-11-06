use disarium::find_disarium;

const BOUND: u64 = 10u64.pow(8);

fn main() {
    for num in find_disarium(BOUND) {
        println!("{num}");
    }
}
