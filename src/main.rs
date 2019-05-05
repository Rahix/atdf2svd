#[macro_use]
pub mod error;

pub mod atdf;
pub mod chip;
pub mod elementext;
pub mod util;

pub use elementext::ElementExt;
pub use error::{Error, Result};

fn main() {
    let b = include_bytes!("../ATtiny85.atdf");

    let chip = atdf::parse(&b[..]).unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(1);
    });

    println!("{:#?}", chip);
}
