fn main() {
    let range1 = 1..=3; //create values 1,2 and 3 ..= will include last value
    let range2 = 1..5; //create values 1,2,3 and 4 .. will not include last value
    for num in range1 {
        println!("{:?}", num);
    }
    println!("------------------");
    for num in range2 {
        println!("{:?}", num);
    }
    println!("------------------");
    //short way to use ranges in loops
    for num in 1..4 {
        println!("{:?}", num);
    }
    println!("------------------");
    for num in 1..4 {
        println!("{:?}", num);
    }
    println!("------------------");
    //range of letters
    for num in 'a'..='f' {
        println!("{:?}", num);
    }
}
