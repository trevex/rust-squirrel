extern crate squirrel;

use std::intrinsics::{type_id};

fn main() {
    println!("{}", unsafe { type_id::<i32>() });
    println!("{}", unsafe { type_id::<squirrel::VM>() });


    let mut vm = squirrel::VM::new();
    let name = "Peter";
    vm.func0("hello", |&:| {
        println!("Hello, {}!", name);
    });
    let mut counter = 0u32;
    vm.func0("foo", |&mut:| {
        counter += 1;
        println!("Counter: {}", counter);
    });
    vm.eval("
        ::hello();\n
        ::foo();\n
        ::foo();\n
    ");
}
