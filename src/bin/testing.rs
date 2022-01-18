fn all_caps(word: &str) -> String {
    word.to_uppercase()
}

fn main() {}

#[cfg(test)] //cfg means configuration
mod test {
    //test module
    use crate::*; //Since this module does not have access to the all_caps method
    #[test]
    fn check_all_caps() {
        let result = all_caps("hello");
        let expected = String::from("HELLO");
        assert_eq!(result, expected, "string should be all upercase");
    }
}
