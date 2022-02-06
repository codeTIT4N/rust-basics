#[derive(Debug)]
enum Answer {
    Yes,
    No,
}

#[derive(Debug)]
//here <'a> is defining the lifetime available for use
struct Form<'a> {
    //to use the lifetime
    question: &'a Answer,
}
fn main() {
    let answer = Answer::Yes;

    //here answer is now owned by form
    let form = Form { question: &answer };

    println!("{:?}", form)
}

#[derive(Debug)]
struct Quiz {
    question: Answer,
}

fn get_first_question<'a>(quiz_1: &'a Quiz, quiz_2: &Quiz) -> &'a Answer {
    &quiz_1.question
}
