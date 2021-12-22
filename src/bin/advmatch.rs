enum Discount {
    Percent(i32),
    Flat(i32),
}

struct Ticket {
    event: String,
    price: i32,
}

fn main() {
    let n = 3;
    match n {
        3 => println!("three"),
        other => println!("number: {:?}", other),
    }

    let flat = Discount::Flat(2); //creating variance with extra data
    match flat {
        Discount::Flat(2) => println!("flat discount of 2"),
        Discount::Flat(amount) => println!("flat discount of {:?}", amount),
        _ => (), //this means we will ignore everything else this is just returning nothing
    }

    let concert = Ticket {
        event: "concert".to_owned(),
        price: 50,
    };
    match concert {
        Ticket { price: 50, event } => println!("event @ 50 = {:?}", event),
        Ticket { price, .. } => println!("price = {:?}", price), //.. means ignore other fields
    }
}
