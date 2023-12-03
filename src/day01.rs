pub fn exercice(content: String) -> i32 {
    return content.len() as i32;
}



#[cfg(test)]
mod tests {
    use crate::day01::exercice;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn should_do_something_for_the_exercice() {
        // Given
        let test_content = String::from("1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet");

        // When/Then
        assert_eq!(exercice(test_content), 40);
    }
}
