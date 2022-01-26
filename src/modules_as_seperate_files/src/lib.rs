pub mod group;
pub mod helper;
pub fn print_from_lib() {
    use helper::{print_again, print_from_helper};
    println!("Hello, from lib");
    print_from_helper();
    print_again();
    group::g1::g1_hello();
}
