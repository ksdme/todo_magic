mod macros;

fn is_even(a: u32) -> bool {
    sprinkle_magic_here!();
}

fn main() {
    println!("is_even {:?}", is_even(3));
}
