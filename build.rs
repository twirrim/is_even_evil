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
    writeln!(&mut nasty_rs, "pub fn is_even(number: u32) -> bool {{").unwrap();
    // Loop!
    for i in 0..=(u32::MAX / 1000) {
        if i % 2 == 0 {
            writeln!(
                &mut nasty_rs,
                "    if number == {i} {{\n        return true; \n    }}\n"
            )
            .unwrap();
        } else {
            writeln!(
                &mut nasty_rs,
                "    if number == {i} {{\n        return false; \n    }}\n"
            )
            .unwrap();
        }
    }
    // Footer
    writeln!(&mut nasty_rs, "    return false;").unwrap();
    writeln!(&mut nasty_rs, "}}").unwrap();

    println!("cargo:rerun-if-changed=build.rs");
}
