use colored::Colorize;
use std::io::Write;

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
    writeln!(stderr).unwrap();
    std::process::exit(1);
}

pub fn panic_with_error(e: crate::Error) -> ! {
    let mut message: Vec<u8> = "Error: ".into();
    e.format(&mut message).unwrap();
    panic!("{}", String::from_utf8_lossy(&message));
}
