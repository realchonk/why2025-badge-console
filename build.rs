fn main() {
    why2025_badge_metadata::generate("Cargo.toml", "target").unwrap();

    println!("cargo:rustc-link-arg=--shared");
    println!("cargo:rustc-link-arg=--retain-symbols-file=retain-symbols.txt");
    println!("cargo:rustc-link-arg=--gc-sections");
    println!("cargo:rustc-link-arg=--strip-debug");
    println!("cargo:rustc-link-arg=--discard-locals");
    println!("cargo:rustc-link-arg=--entry=main");
}
