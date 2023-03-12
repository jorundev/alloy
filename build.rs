use std::time::Duration;

/*
	Cargo allows to build staticlib crates but not linking to them.
	This build script waits for the library to be built, then links to it.
 */
fn main() {
	// Passed from Makefile. Directory where the dependencies are built
    let bootcore_dir = std::env::var("LIBBOOTCORE_DIR").unwrap();
    let mut bootcore = None;

    loop {
        let paths = std::fs::read_dir(&bootcore_dir).unwrap();
        for path in paths {
            let str = String::from(path.unwrap().path().to_str().unwrap());
            let start = format!("{}/libbootcore", bootcore_dir);

			// Check if there is a file starting with "$LIBBOOTCORE_DIR/libbootcore", ending with ".a"
			// and not containing "temp"
            if str.starts_with(&start) && str.ends_with(".a") && !str.contains("temp") {
                let final_str = &str[(bootcore_dir.len() + 4)..];
                let final_str = &final_str[..(final_str.len() - 2)];
                bootcore = Some(String::from(final_str));
                break;
            }
        }

        if !bootcore.is_none() {
            break;
        }

        // 100ms sleep to not make too many io reads
        std::thread::sleep(Duration::from_millis(100));
    }

	// If we are out of the loop then bootcore is not None
	let bootcore = bootcore.unwrap();

	// TODO: depends on target
    println!("cargo:rerun-if-changed=layouts/x86.ld");

    println!("cargo:rustc-link-search={bootcore_dir}");
    println!("cargo:rustc-link-lib={}", bootcore);
}
