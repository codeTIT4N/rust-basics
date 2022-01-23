enum Color {
    Red,
    Orange,
    Blue,
}
fn main() {
    let maybe_user = Some("Lokesh");
    if let Some(user) = maybe_user {
        println!("user= {:?}", user);
    } else {
        println!("no user")
    }

    if let red = Color::Red {
        println!("its red!!!");
    } else {
        println!("its not red!!!");
    }
}
