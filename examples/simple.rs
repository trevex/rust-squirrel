extern crate squirrel;

fn main() {
    let mut vm = squirrel::VM::new();
    vm.func0("hello", || {
        println!("Hello, world!");
    });
    vm.eval("
        ::hello();
    ");
}
