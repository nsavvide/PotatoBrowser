fn main() {
    // Tell Cargo to look for libs here
    println!("cargo:rustc-link-search=native=cef_binary/Release");

    // Link to the shared CEF library
    println!("cargo:rustc-link-lib=dylib=cef");

    // Optional: Link to X11 libraries (for Linux GUI support)
    println!("cargo:rustc-link-lib=dylib=X11");
}
