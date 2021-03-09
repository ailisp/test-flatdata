fn main() {
    flatdata::generate(
        "assets/wasmer_cache.flatdata",
        &std::env::var("OUT_DIR").unwrap(),
    )
    .expect("generator failed");
}
