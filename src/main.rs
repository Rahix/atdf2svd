pub use gumdrop::Options;

fn main() {
    let args = atdf2svd::Atdf2SvdOptions::parse_args_default_or_exit();
    atdf2svd::run(args);
}
