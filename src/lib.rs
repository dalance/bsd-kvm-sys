#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    use std::ptr;

    #[test]
    fn it_works() {
        let corefile = CString::new("/dev/null").unwrap();
        let kvm = unsafe {
            kvm_open(
                ptr::null(),
                corefile.as_ptr() as *const i8,
                ptr::null(),
                0,
                ptr::null(),
            )
        };
        let mut ret: i32 = 0;
        let procs = unsafe { kvm_getprocs(kvm, 8, 0, &mut ret) };
        let ret = ret as usize;
        let procs = unsafe { std::slice::from_raw_parts(procs, ret) };
        for i in 0..ret {
            dbg!(procs[i].ki_pid);
        }
        assert_eq!(ret, 0);
    }
}
