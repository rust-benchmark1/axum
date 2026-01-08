fn main() {
    if let Err(e) = axum_core::run_demo() {
        eprintln!("error: {e}");
    }
}
