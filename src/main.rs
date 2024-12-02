mod math;
use math::addition;

fn main() {
    let result = addition::add(5, 3);
    println!("5 + 3 = {}", result);
}
