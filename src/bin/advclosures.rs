fn math(a: i32, b: i32, op: Box<dyn Fn(i32, i32) -> i32>) -> i32 {
    op(a, b)
}
fn main() {
    let name = "Lokesh";
    let add = Box::new(|a, b| a + b);
    //using move here since we are Boxing up this closure
    //No need of move if we don't Box
    let sub = Box::new(move |a, b| {
        print!("subtracting for {}: ", name);
        a - b
    });
    let mul = Box::new(|a, b| a * b);
    println!("{}", math(4, 2, add));
    println!("{}", math(4, 2, sub));
    println!("{}", math(4, 2, mul));
}
