use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "1 2 3\n4 5 6";
        let expected = vec![vec![1, 2, 3], vec![4, 5, 6]];

        let result = parse_input(input);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_parse_input_empty() {
        let input = "";
        let expected: Vec<Vec<u32>> = Vec::new();

        let result = parse_input(input);

        assert_eq!(expected, result);
    }
}
