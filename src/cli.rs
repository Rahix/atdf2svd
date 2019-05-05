pub fn setup(verbose: bool) {
    env_logger::Builder::new()
        .filter_level(if verbose {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Warn
        })
        .format(|f, r| {
            use colored::Colorize;
            use log::Level::*;
            use std::io::Write;

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
