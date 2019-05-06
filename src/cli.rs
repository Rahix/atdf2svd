use std::io::Write;
use colored::Colorize;

pub fn setup(verbose: bool) {
    env_logger::Builder::new()
        .filter_level(if verbose {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Warn
        })
        .format(|f, r| {
            use log::Level::*;

            writeln!(
                f,
                "{}: {}",
                match r.level() {
                    Warn => "Warning".yellow().bold(),
                    Error => "Error".red().bold(),
                    Info => "Info".bold(),
                    Debug => "Debug".dimmed(),
                    Trace => "Trace".dimmed(),
                },
                r.args()
            )
        })
        .init();
}

pub fn exit_with_error(e: crate::Error) -> ! {
    let mut stderr = std::io::stderr();
    write!(stderr, "{}: ", "Error".red().bold()).unwrap();
    e.format(&mut stderr).unwrap();
    write!(stderr, "\n").unwrap();
    std::process::exit(1);
}
