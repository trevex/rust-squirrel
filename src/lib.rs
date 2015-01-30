#![feature(libc)]
extern crate libc;

#[allow(dead_code, non_camel_case_types, raw_pointer_derive, non_snake_case, missing_copy_implementations)]
mod ffi;

pub struct VM {
    raw: ffi::HSQUIRRELVM,
}

impl VM {
    pub fn new() -> VM {
        unsafe {
            let mut vm = ffi::sq_open(1024);
            ffi::sq_helper_setup_default_callbacks(vm);
            VM { raw: vm }
        }
    }
}

impl Drop for VM {
    fn drop(&mut self) {
        unsafe {
            ffi::sq_close(self.raw);
        }
    }
}

#[test]
fn starting_squirrel() {
    let vm = VM::new();
}
