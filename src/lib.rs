#[macro_use]
pub mod error;
#[cfg(feature = "cli")]
pub mod cli;

pub mod atdf;
pub mod chip;
pub mod elementext;
pub mod svd;
pub mod util;

pub use elementext::ElementExt;
pub use error::{DisplayError, Error, Result};
#[cfg(feature = "cli")]
pub use gumdrop::Options;
use std::collections::HashSet;
use std::iter::FromIterator;

#[cfg(feature = "cli")]
#[derive(Debug, Options)]
/// A tool to convert AVR chip description files (.atdf) to SVD.
pub struct Atdf2SvdOptions {
    /// Path to the .atdf file to convert
    #[options(free)]
    atdf_path: Option<std::path::PathBuf>,

    /// [optional] Path where to save the SVD file
    #[options(free)]
    svd_path: Option<std::path::PathBuf>,

    /// List of patches to apply.
    #[options(long = "auto-patches")]
    auto_patches: Vec<String>,

    #[options(short = "d", long = "debug")]
    debug: bool,

    #[options(short = "v", long = "verbose")]
    verbose: bool,

    help: bool,

    #[options(short = "V", long = "version")]
    version: bool,
}

#[cfg(feature = "cli")]
pub fn run(args: Atdf2SvdOptions) {
    if args.version {
        println!(
            "{} {}",
            env!("CARGO_PKG_NAME"),
            git_version::git_version!(
                args = ["--always", "--dirty", "--abbrev=12"],
                cargo_prefix = "v",
                cargo_suffix = " (no git)",
                fallback = "unknown"
            ),
        );
        return;
    }

    cli::setup(args.verbose);

    let atdf_file = if let Some(atdf_path) = args.atdf_path {
        std::fs::File::open(atdf_path).unwrap_or_else(|e| cli::exit_with_error(e.into()))
    } else {
        log::error!("Missing atdf-file argument");
        std::process::exit(1);
    };

    let svd_file: Box<dyn std::io::Write> = if let Some(p) = args.svd_path {
        Box::new(std::fs::File::create(p).unwrap_or_else(|e| cli::exit_with_error(e.into())))
    } else {
        Box::new(std::io::stdout())
    };

    let patches = HashSet::from_iter(args.auto_patches.iter().cloned());
    let chip = atdf::parse(atdf_file, &patches).unwrap_or_else(|e| cli::exit_with_error(e));

    if args.debug {
        eprintln!("{:#?}", chip);
    }

    svd::generate(&chip, svd_file).unwrap_or_else(|e| cli::exit_with_error(e));
}

pub fn run_test(atdf: &mut dyn std::io::Read, auto_patches: Vec<&str>) -> String {
    let patches = HashSet::from_iter(auto_patches.iter().map(|s| s.to_string()));
    let chip = atdf::parse(atdf, &patches).unwrap_or_else(|e| e.to_panic());
    let mut output = Vec::new();
    svd::generate(&chip, &mut output).unwrap_or_else(|e| e.to_panic());
    String::from_utf8(output).unwrap()
}
