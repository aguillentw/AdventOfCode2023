pub fn exercice1(content: String) -> u32 {
    let mut result = 0;

    let lines = content.split("\n");
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let calibrated_value = find_calibrated_value(line.to_string());
        result += calibrated_value;
    }

    return result;
}

pub fn exercice2(content: String) -> u32 {
    let mut result = 0;

    let lines = content.split("\n");
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let calibrated_value = find_calibrated_value(replace_digit_words_by_their_value(line.to_string()));
        result += calibrated_value;
    }
    return result;
}

fn find_first_digit(content: String) -> char {
    let result = content.find(|c: char| c.is_ascii_digit()).unwrap();
    return content.chars().nth(result).unwrap();
}

fn find_last_digit(content: String) -> char {
    let result = content.rfind(|c: char| c.is_ascii_digit()).unwrap();
    return content.chars().nth(result).unwrap();
}

fn find_calibrated_value(content: String) -> u32 {
    let first_digit = find_first_digit(content.clone());
    let last_digit = find_last_digit(content.clone());

    let first_digit_value = first_digit.to_digit(10).unwrap();
    let last_digit_value = last_digit.to_digit(10).unwrap();

    return (first_digit_value * 10) + last_digit_value;
}

fn replace_digit_words_by_their_value(content: String) -> String {
    // replace digit words by their value for mixed words
    // e.g. eightwothree to e8t2ot3e
    let mut content = content.replace("one", "o1e");
    content = content.replace("two", "t2o");
    content = content.replace("three", "t3e");
    content = content.replace("four", "f4r");
    content = content.replace("five", "f5e");
    content = content.replace("six", "s6x");
    content = content.replace("seven", "s7n");
    content = content.replace("eight", "e8t");
    content = content.replace("nine", "n9e");
    content = content.replace("zero", "z0o");
    return content;
}


#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(exercice1(test_content), 142);
    }

    #[test]
    fn should_find_the_first_digit_in_a_string() {
        // Given
        let test_content = String::from("asdfa91abc2");

        // When
        let result = find_first_digit(test_content);

        // Then
        assert_eq!(result, '9');
    }

    #[test]
    fn should_find_the_last_digit_in_a_string() {
        // Given
        let test_content = String::from("asdfa91abc2");

        // When
        let result = find_last_digit(test_content);

        // Then
        assert_eq!(result, '2');
    }

    #[test]
    fn should_find_the_calibrated_value() {
        // Given
        let test_content = String::from("asdfa91abc2assdf");

        // When
        let result = find_calibrated_value(test_content);

        // Then
        assert_eq!(result, 92);
    }

    #[test]
    // Test to check if the function replace digit words by their value
    // is able to replace the digit words by their value for mixed words
    // like eightwothree to 823
    fn should_replace_digit_words_by_their_value_for_mixed_words() {
        // Given
        let test_content = String::from("eightwothree");

        // When
        let result = replace_digit_words_by_their_value(test_content);

        // Then
        assert_eq!(result, String::from("e8t2ot3e"));
    }

}
