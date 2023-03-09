// ARM is not yet supported by the kernel. This is just a placeholder.
#[cfg(all(feature = "x86", feature = "arm"))]
compile_error!("Features 'x86' and 'arm' are mutually exclusive");

fn main() {
    println!();
}
