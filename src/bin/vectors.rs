struct Test {
    score: i32,
}

fn main() {
    let my_scores = vec![
        Test { score: 80 },
        Test { score: 70 },
        Test { score: 82 },
        Test { score: 90 },
        Test { score: 93 },
    ]; //don't forget the ; here

    for test in my_scores {
        println!("score = {:?}", test.score);
    }
}
