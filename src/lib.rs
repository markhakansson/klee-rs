#![no_std]

extern crate klee_bindings as ffi;

use cty::{c_char, c_void};

/// Makes the object `var` symbolic. `name` is the identifier of this object
/// used by KLEE. Does not terminate the C string.
#[inline(always)]
pub fn klee_make_symbolic<T>(var: &mut T, name: &str) {
    unsafe {
        ffi::klee_make_symbolic(
            var as *mut T as *mut c_void,
            core::mem::size_of::<T>() as ffi::size_t,
            name.as_ptr() as *const c_char,
        )
    }
}

/// Abort the KLEE instance.
#[inline(always)]
pub fn klee_abort() {
    unsafe {
        ffi::klee_abort();
    }
}

/// Assumes that the `condition` is true during KLEE runtime.
#[inline(always)]
pub fn klee_assume(condition: bool) {
    unsafe {
        ffi::klee_assume(condition as usize);
    }
}
