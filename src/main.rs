mod thing;
use thing::inner_get_num;

fn main() {
    println!("Hello, world!");
    let x = inner_get_num();
    println!("x: {}", x);
}
