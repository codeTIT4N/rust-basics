use demo::group;
use demo::print_from_lib; //due to toml file

fn main() {
    print_from_lib();
    group::g1::g1_hello();
}
