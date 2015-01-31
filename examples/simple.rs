extern crate squirrel;

fn main() {
    let mut vm = squirrel::VM::new();
    let name = "Peter";
    vm.func0("hello", |&:| {
        println!("Hello, {}!", name);
    });
    vm.eval("
        ::hello();
    ");
}
