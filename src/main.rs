fn main() {
    if let Err(e) = wicker::get_args().and_then(wicker::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
