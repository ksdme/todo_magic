mod macros;

fn sum(a: u32, b: u32) -> u64 {
    sprinkle_magic_here!();
}

fn is_even(a: u32) -> bool {
    sprinkle_magic_here!();
}

fn string_reverse(string: String) -> String {
    sprinkle_magic_here!();
}

fn main() {
    println!("is_even {:?}", is_even(3));
    println!("sum {:?}", sum(1, 3));
    println!("string_reverse {:?}", string_reverse("Hello World".to_owned()));
}
