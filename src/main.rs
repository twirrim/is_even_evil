use std::env;

include!(concat!(env!("OUT_DIR"), "/nasty.rs"));

fn main() {
    let args: Vec<_> = env::args().collect();
    let to_evaluate: u32 = args[1].parse().expect("Can't parse that in to a valid number type for this program");
    if to_evaluate <= (u32::MAX / 1000) {
        if is_even(to_evaluate) {
            println!("even");
        } else {
            println!("odd");
        };
    } else {
        println!("Sorry, input number is too big");
    }
}
