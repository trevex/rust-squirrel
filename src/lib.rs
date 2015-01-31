#![feature(libc)]
extern crate libc;

use std::ffi::CString;
use std::mem;
use std::ptr;

#[allow(dead_code, non_camel_case_types, raw_pointer_derive, non_snake_case, missing_copy_implementations)]
mod ffi;

mod wrapper {
    use ::std::mem;
    use ::ffi;

    pub extern "C" fn func0<F, R>(vm: ffi::HSQUIRRELVM) -> ffi::SQInteger where F: Fn() -> R {
        unsafe {
            let mut ptr: ffi::SQUserPointer;
            ffi::sq_getuserdata(vm, -1, mem::transmute(&ptr), mem::transmute(0)); // TODO: last arguments should be NULL, find better way
            let func: F = mem::transmute(ptr);
            //let func: F = ptr as F;
            func();
        }
        0
    }
}

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

    pub fn func0<F, R>(&mut self, name: &str, func: F) where F: Fn() -> R {
        self.push_function(name, func, wrapper::func0::<F, R>);
    }

    fn push_function<F>(&mut self, name: &str, func: F, wrapper: extern "C" fn(ffi::HSQUIRRELVM) -> ffi::SQInteger) {
        let c_str = CString::from_slice(name.as_bytes());
        unsafe {
            ffi::sq_pushroottable(self.raw);
            ffi::sq_pushstring(self.raw, c_str.as_ptr(), -1);
            let user_ptr = ffi::sq_newuserdata(self.raw, mem::size_of::<F>() as ffi::SQUnsignedInteger);
            ptr::write(mem::transmute(user_ptr), func);
            ffi::sq_newclosure(self.raw, Some(wrapper), 0);
            ffi::sq_newslot(self.raw, -3, 0);
            ffi::sq_pop(self.raw, 1);
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
    let mut vm = VM::new();
    vm.func0("test", || {
        println!("Hello, world!");
    });
}
