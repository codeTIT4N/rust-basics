enum Answer {
    Yes,
    No,
}

fn main() {
    let yes = Answer::Yes;
    let yes_heap: Box<Answer> = Box::new(yes);
    let yes_stack = *yes_heap;
}
