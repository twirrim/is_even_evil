use std::env;

include!(concat!(env!("OUT_DIR"), "/nasty.rs"));

fn main() {
    let args: Vec<_> = env::args().collect();
    let to_evaluate: u16 = args[1].parse().expect("Can't parse that in to a valid number type for this program");
    if is_even(to_evaluate) {
        println!("even");
    } else {
        println!("odd");
    };
}
