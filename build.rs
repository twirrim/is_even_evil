// build.rs

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("nasty.rs");
    let mut nasty_rs = File::create(&dest_path).expect("Cannot write to file");
    // Header
    writeln!(&mut nasty_rs, "pub fn is_even(number: u16) -> bool {{").unwrap();
    writeln!(&mut nasty_rs, "   match number  {{").unwrap();
    // Loop!
    for i in 0..=u16::MAX {
        if i % 2 == 0 {
            writeln!(&mut nasty_rs, "  {i}  =>  true,    ").unwrap();
        } else {
            writeln!(&mut nasty_rs, "  {i}  =>  false,    ").unwrap();
        }
    }
    // Footer
    writeln!(&mut nasty_rs, "   }}").unwrap();
    writeln!(&mut nasty_rs, " }}").unwrap();

    println!("cargo:rerun-if-changed=build.rs");
}
