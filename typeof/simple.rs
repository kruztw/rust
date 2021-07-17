// https://stackoverflow.com/questions/21747136/how-do-i-print-the-type-of-a-variable-in-rust

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    let s = "Hello";
    let i = 42;

    print_type_of(&s); // &str
    print_type_of(&i); // i32
    print_type_of(&main); // simple::main
    print_type_of(&print_type_of::<i32>); // simple::print_type_of<i32>
    print_type_of(&{ || "Hi!" }); // simple::main::{{closure}}
}
