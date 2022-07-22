// run-rustfix
#![warn(clippy::assert_ok)]

fn main() {
    // basic case
    let r: std::result::Result<(), ()> = Ok(());
    assert!(r.is_ok());
}
