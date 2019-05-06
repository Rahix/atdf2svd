#[macro_use]
pub mod error;
pub mod cli;

pub mod atdf;
pub mod chip;
pub mod elementext;
pub mod svd;
pub mod util;

pub use elementext::ElementExt;
pub use error::{DisplayError, Error, Result};

#[derive(Debug, structopt::StructOpt)]
struct Options {
    #[structopt(parse(from_os_str))]
    atdf_path: std::path::PathBuf,

    #[structopt(parse(from_os_str))]
    svd_path: Option<std::path::PathBuf>,

    #[structopt(short = "d", long = "debug")]
    debug: bool,

    #[structopt(short = "v", long = "verbose")]
    verbose: bool,
}

fn main() {
    let args: Options = structopt::StructOpt::from_args();

    cli::setup(args.verbose);

    let atdf_file =
        std::fs::File::open(args.atdf_path).unwrap_or_else(|e| cli::exit_with_error(e.into()));
    let svd_file: Box<dyn std::io::Write> = if let Some(p) = args.svd_path {
        Box::new(std::fs::File::create(p).unwrap_or_else(|e| cli::exit_with_error(e.into())))
    } else {
        Box::new(std::io::stdout())
    };

    let chip = atdf::parse(atdf_file).unwrap_or_else(|e| cli::exit_with_error(e.into()));

    if args.debug {
        eprintln!("{:#?}", chip);
    }

    svd::generate(&chip, svd_file).unwrap_or_else(|e| cli::exit_with_error(e.into()));
}
