fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    //without using iterator
    // let mut plus_one = vec![];
    // for num in numbers {
    //     plus_one.push(num + 1);
    // }
    //using iterator with map
    let plus_one: Vec<_> = numbers.iter().map(|num| num + 1).collect();
    //using some other functions with iterators
    let new_numbers: Vec<_> = numbers.iter().filter(|num| num <= &&3).collect();
    let find_me: Option<_> = numbers.iter().find(|num| num == &&3); //finds 3
    let count = numbers.iter().count(); //counts number of elements
    let last: Option<_> = numbers.iter().last(); //returns last number
    let min: Option<_> = numbers.iter().min(); //return minimum number
    let max: Option<_> = numbers.iter().max(); //return maximum number
    let take: Vec<_> = numbers.iter().take(3).collect(); //takes first 3 items from vector

    //mix examples:
    let plus_one: Vec<_> = numbers
        .iter()
        .map(|num| num + 1)
        .filter(|num| num < &1)
        .collect();
}
