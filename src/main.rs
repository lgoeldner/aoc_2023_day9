use aoc_2023_day9::*;

#[rustfmt::skip]
const TEST_INPUT: &str = 
"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

fn main() {
    let parsed = parse_input(TEST_INPUT);
    dbg!(&parsed);
    for line in parsed {
        let vec = line
            .as_slice()
            .windows(2)
            .map(|pair| pair[0].abs_diff(pair[1]))
            .collect::<Vec<_>>();
        dbg!(vec);
        todo!();
    }
}
