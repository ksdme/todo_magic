#[macro_export]
macro_rules! todo_magic {
    () => {
        include!(concat!(env!("OUT_DIR"), "/sprinkles/", line!()))
    };
}
