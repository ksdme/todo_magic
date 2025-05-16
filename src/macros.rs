#[macro_export]
macro_rules! sprinkle_magic_here {
    () => {
        include!(concat!(env!("OUT_DIR"), "/sprinkles/", line!()))
    };
}
