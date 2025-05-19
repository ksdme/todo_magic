mod macros;

fn is_even(a: u32) -> bool {
    todo_magic!();
}

fn reverse_a_string(a: &str) -> String {
    todo_magic!();
}

fn pluralize(count: usize, a: &str) -> String {
    todo_magic!();
}

fn main() {
    println!("is_even {:?}", is_even(3));
    println!("reverse seting {:?}", reverse_a_string("Hello World"));
    println!("reverse seting {:?}", pluralize(0, "ant"));
}
