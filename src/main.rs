use aoc_2023_day9::*;

fn main() {
    let parsed = parse_input(&std::fs::read_to_string("data.txt").unwrap());
    dbg!(&parsed);
    let mut sum = 0;

    for line in parsed {
        let result = get_next_prediction(&line);
        sum += result;
        dbg!(result);
    }

    println!("sum: {}", sum);
}

#[test]
fn with_test_input() {
	let parsed = parse_input(TEST_INPUT);
	
	let mut sum = 0;
    for line in parsed {
        let result = get_next_prediction(&line);
        sum += result;
        dbg!(result);
    }

	println!("sum: {}", sum);
	assert_eq!(sum, 114);
}
