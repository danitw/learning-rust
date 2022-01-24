// Discovery type of variables and functions
// Functions have types(asignatures)


fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    let s = "Hello";
    let i = 42;

    print_type_of(&s); // &str
    print_type_of(&i); // i32
    print_type_of(&main); // playground::main
    print_type_of(&print_type_of::<i32>); // playground::print_type_of<i32>
    print_type_of(&{ || "Hi!" }); // playground::main::{{closure}}

    fn foo(x: i32) -> i32 { unimplemented!("WIP") };
    let x: fn(i32) -> i32 = foo;
    print_type_of(&foo);
    print_type_of(&x);

    x(12);
}
