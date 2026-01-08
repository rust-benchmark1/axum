fn main() {
    if let Err(e) = axum::run_demo() {
        eprintln!("error: {e}");
    }
}
