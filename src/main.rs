mod autocomplete;

fn main() {
    println!(
        "{:?}",
        autocomplete::complete_code("pub fn add_two_numbers(a: u16, b: u32) -> u32 {}")
    );
}
