mod greet {
    fn hello() {
        println!("hello");
    }

    fn goodbye() {
        println!("goodbye");
    }
}
mod math {
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    fn sub(a: i32, b: i32) -> i32 {
        a - b
    }
}
fn main() {
    use greet::hello;
    hello();
    greet::goodbye();
    let ans = math::add(2, 1);
    println!("{:?}", ans);
}
//Note this will still give error as all the functions are private by default.
