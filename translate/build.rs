fn main() {
    println!("cargo:rustc-link-search=framework=/System/Library/Frameworks");
    println!("cargo:rustc-link-lib=framework=SystemConfiguration");
}
