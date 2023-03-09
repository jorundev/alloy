fn main() {
    let dir = std::env::var("LIBBOOTCORE_DIR").unwrap();
	println!("cargo:rerun-if-changed=layouts/x86.ld");
    println!("cargo:rustc-link-search={dir}");
    println!("cargo:rustc-link-lib={}", "bootcore");
}
