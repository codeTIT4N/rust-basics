enum Access {
    Admin,
    Manager,
    User,
    Guest,
}

fn main() {
    //secret file: admins only
    let access_level = Access::Guest;
    let can_access_file = match access_level {
        Access::Admin => true,
        _ => false,
    };
    println!("{:?}", can_access_file)
}

// fn main() {
//     // let my_num = 3;
//     // let is_lt_5 = if my_num < 5 { true } else { false };
//     // let is_lt_5 = my_num < 5; //shorthand

//     // let message = match my_num {
//     //     1 => "hello",
//     //     _ => "bye",
//     // };
//     // println!("{:?}", message);

// }
