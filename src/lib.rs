#[cfg(test)]
mod test_parse_input;

#[rustfmt::skip]
pub const TEST_INPUT: &str =
"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

pub fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect()
        })
        .collect()
}

/// get next prediction recursively
pub fn get_next_prediction(input: &Vec<i64>) -> i64 {
    fn recurse_diffvec_helper(input: &Vec<i64>) -> i64 {
        // go through each pair of numbers in the vector and find the difference

        let vec = input
            .as_slice()
            .windows(2)
            .map(|pair| pair[0].abs_diff(pair[1]).try_into().unwrap())
            .collect::<Vec<_>>();

        // if all the differences are 0, return 0 (the last element)
        // otherwise return the last difference + the result of the recursive call
        if vec.iter().all(|x| *x == 0) {
            return 0;
        } else {
            return vec.last().unwrap() + recurse_diffvec_helper(&vec);
        }
    }

    input.last().unwrap() + recurse_diffvec_helper(input)
}

#[cfg(test)]
mod tests {

    mod diffvec {
        use crate::*;
        #[test]
        fn test_diff_vec1() {
            let data = "0 3 6 9 12 15";
            let parsed = parse_input(data);
            let result = get_next_prediction(&parsed[0]);
            assert_eq!(result, 18);
        }

        #[test]
        fn test_diff_vec2() {
            let data = TEST_INPUT.lines().nth(1).unwrap();
            let parsed = parse_input(data);
            let result = get_next_prediction(&parsed[0]);
            assert_eq!(result, 28);
        }

        #[test]
        fn test_diff_vec3() {
            let data = TEST_INPUT.lines().nth(2).unwrap();
            let parsed = parse_input(data);
            let result = get_next_prediction(&parsed[0]);
            assert_eq!(result, 68);
        }
    }
}
