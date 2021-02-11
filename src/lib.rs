extern crate klee_bindings as ffi;
extern crate libc;

use libc::{c_char, c_void};

pub fn klee_make_symbolic<T>(var: &mut T, name: String) {
    unsafe {
        ffi::klee_make_symbolic(
            var as *mut T as *mut c_void,
            std::mem::size_of::<T>() as ffi::size_t,
            name.as_ptr() as *const c_char,
        )
    }
}

pub fn klee_abort() {
    unsafe {
        ffi::klee_abort();
    }
}

pub fn klee_assume(condition: bool) {
    unsafe {
        ffi::klee_assume(condition as usize);
    }
}
