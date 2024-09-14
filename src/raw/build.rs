fn main() {
    cc::Build::new().cpp(true).file("raw.cpp").compile("raw");
    println!("cargo:rerun-if-changed=raw.cpp");
}
