use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to look for shared libraries in the specified directory
    //println!("cargo:rustc-link-search=/path/to/lib");

    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    //println!("cargo:rustc-link-lib=bz2");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .allowlist_function("kvm_close")
        .allowlist_function("kvm_dpcpu_setcpu")
        .allowlist_function("kvm_getargv")
        .allowlist_function("kvm_getcptim")
        .allowlist_function("kvm_getenvv")
        .allowlist_function("kvm_geterr")
        .allowlist_function("kvm_getfiles")
        .allowlist_function("kvm_getloadavg")
        .allowlist_function("kvm_getmaxcpu")
        .allowlist_function("kvm_getpcpu")
        .allowlist_function("kvm_counter_u64_fetch")
        .allowlist_function("kvm_getprocs")
        .allowlist_function("kvm_getswapinfo")
        .allowlist_function("kvm_nlist")
        .allowlist_function("kvm_open")
        .allowlist_function("kvm_openfiles")
        .allowlist_function("kvm_read")
        .allowlist_function("kvm_read_zpcpu")
        .allowlist_function("kvm_write")
        .allowlist_var("_SIG_WORDS")
        .allowlist_var("CRED_FLAG_CAPMODE")
        .allowlist_var("PRI_ITHD")
        .allowlist_var("PRI_REALTIME")
        .allowlist_var("PRI_TIMESHARE")
        .allowlist_var("PRI_IDLE")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
