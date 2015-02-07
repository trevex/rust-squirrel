#![feature(libc, std_misc)]
extern crate libc;

use std::ffi::CString;
use std::mem;
use std::ptr;
use std::collections::{HashMap};

#[allow(dead_code, non_camel_case_types, raw_pointer_derive, non_snake_case, missing_copy_implementations)]
mod ffi;

mod wrapper {
    use ::std::mem;
    use ::ffi;
    use ::libc;

    pub extern "C" fn func0<F, R>(vm: ffi::HSQUIRRELVM) -> ffi::SQInteger where F: FnMut() -> R {
        unsafe {
            let ptr: ffi::SQUserPointer = ffi::sq_helper_get_null();
            ffi::sq_getuserdata(vm, -1, mem::transmute(&ptr), ffi::sq_helper_get_null() as *mut ffi::SQUserPointer);
            let func_ptr: *mut F = ptr as *mut libc::c_void as *mut F;
            (*func_ptr)();
        }
        0
    }
}

pub struct VM {
    raw: ffi::HSQUIRRELVM,
    registered_classes: HashMap<u64, ClassData>,
}

impl VM {
    pub fn new() -> VM {
        unsafe {
            let vm = ffi::sq_open(1024);
            ffi::sq_helper_setup_default_callbacks(vm);
            VM {
                raw: vm,
                registered_classes: HashMap::new(),
            }
        }
    }

    pub fn eval(&mut self, source: &str) {
        let c_source = CString::from_slice(source.as_bytes());
        unsafe {
            ffi::sq_helper_eval(self.raw, c_source.as_slice_with_nul().as_ptr());
        }
    }

    pub fn func0<F, R>(&mut self, name: &str, func: F) where F: FnMut() -> R {
        self.push_function(name, func, wrapper::func0::<F, R>);
    }

    fn push_function<F>(&mut self, name: &str, func: F, wrapper: extern "C" fn(ffi::HSQUIRRELVM) -> ffi::SQInteger) {
        let c_name = CString::from_slice(name.as_bytes());
        unsafe {
            ffi::sq_pushroottable(self.raw);
            ffi::sq_pushstring(self.raw, c_name.as_slice_with_nul().as_ptr(), -1);
            let user_ptr = ffi::sq_newuserdata(self.raw, mem::size_of::<F>() as ffi::SQUnsignedInteger);
            ptr::write(mem::transmute(user_ptr), func);
            //ffi::sq_settypetag(self.raw, -1, TypeId::of<F>());
            ffi::sq_newclosure(self.raw, Some(wrapper), 1);
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

#[allow(missing_copy_implementations)]
pub struct ClassData {
    object: ffi::HSQOBJECT,
    get_table: ffi::HSQOBJECT,
    set_table: ffi::HSQOBJECT,
}

#[test]
fn starting_squirrel() {
    let mut vm = VM::new();
    vm.func0("what", || {
        println!("Hello, world!");
    });
    vm.eval("::what();");
}
