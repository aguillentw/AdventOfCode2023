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
        let calibrated_value = find_calibrated_value(replace_first_and_last_digit_words(line.to_string()));
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

fn replace_first_and_last_digit_words(content: String) -> String {
    let number_words = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut new_content = content.clone();

    for (i, word) in number_words.iter().enumerate() {
        if new_content.contains(word) {
            new_content = new_content.replacen(word, &i.to_string(), 1);
            break;
        }
    }

    for (i, word) in number_words.iter().enumerate().rev() {
        if new_content.contains(word) {
            new_content = new_content.replacen(word, &i.to_string(), 1);
            break;
        }
    }

    new_content
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

    fn should_replace_the_first_and_last_digit_words_by_their_value() {
        // Given
        let test_content = String::from("asdfaeightwoeighthreeasf");

        // When
        let result = replace_first_and_last_digit_words(test_content);

        // Then
        assert_eq!(result, String::from("asdfa8woeigh3asf"));
    }

}
