pub fn exercice1(_content: String) -> u32 {
    let result = 0;
    return result;
}

pub fn exercice2(_content: String) -> u32 {
    let result = 0;
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inputs_are_integers() {
        let input1 = "123";
        let input2 = "456";

        assert!(input1.parse::<i32>().is_ok());
        assert!(input2.parse::<i32>().is_ok());
    }
}
