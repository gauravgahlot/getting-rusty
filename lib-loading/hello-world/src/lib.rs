// The no_mangle attribute turns off Rust's name mangling,
// so that it has a well defined symbol to link to.
#[no_mangle]
pub fn execute() {
    println!("Hello, world!")
}
