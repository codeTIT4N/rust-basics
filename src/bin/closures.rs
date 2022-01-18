// fn add_fn(a: i32, b: i32) -> i32 {
//     //simple function
//     a + b
// }

fn main() {
    let add = |a, b| a + b;
    let sum = add(1, 2);
    println!("{:?}", sum);
}
