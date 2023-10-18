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
        .allowlist_var("O_RDONLY")
        .allowlist_var("O_WRONLY")
        .allowlist_var("O_RDWR")
        .allowlist_var("KERN_PROC_ALL")
        .allowlist_var("KERN_PROC_PROC")
        .allowlist_var("KERN_PROC_PID")
        .allowlist_var("KERN_PROC_PGRP")
        .allowlist_var("KERN_PROC_SESSION")
        .allowlist_var("KERN_PROC_TTY")
        .allowlist_var("KERN_PROC_UID")
        .allowlist_var("KERN_PROC_RUID")
        .allowlist_var("KERN_PROC_INC_THREAD")
        .allowlist_var("_SIG_WORDS")
        .allowlist_var("KI_NGROUPS")
        .allowlist_var("TDNAMLEN")
        .allowlist_var("WMESGLEN")
        .allowlist_var("LOGNAMELEN")
        .allowlist_var("LOCKNAMELEN")
        .allowlist_var("COMMLEN")
        .allowlist_var("KI_EMULNAMELEN")
        .allowlist_var("LOGINCLASSLEN")
        .allowlist_var("MAXCOMLEN")
        .allowlist_var("_POSIX2_LINE_MAX")
        .allowlist_var("TDF_SINTR")
        .allowlist_var("SIDL")
        .allowlist_var("SRUN")
        .allowlist_var("SSLEEP")
        .allowlist_var("SSTOP")
        .allowlist_var("SZOMB")
        .allowlist_var("SWAIT")
        .allowlist_var("SLOCK")
        .allowlist_var("P_INMEM")
        .allowlist_var("P_TRACED")
        .allowlist_var("P_WEXIT")
        .allowlist_var("P_PPWAIT")
        .allowlist_var("P_SYSTEM")
        .allowlist_var("CRED_FLAG_CAPMODE")
        .allowlist_var("KI_SLEADER")
        .allowlist_var("P_CONTROLT")
        .allowlist_var("P_JAILED")
        .allowlist_var("NZERO")
        .allowlist_var("PRI_REALTIME")
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
