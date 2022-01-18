fn maybe_num() -> Option<i32> {
    return Some(2);
}

fn maybe_word() -> Option<String> {
    return None;
}

fn main() {
    let word_length = maybe_word().map(|word| word.len()).map(|len| len * 2);
    println!("length {:?}", word_length);

    let plus_one = maybe_num().map(|num| num + 1);
    println!("num+1 {:?}", plus_one);
}
