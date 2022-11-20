fn main() {
    if let Err(e) = grepr::get_args().and_then(grepr::run) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
