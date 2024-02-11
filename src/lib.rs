mod test_parse_input;

pub fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}

pub fn get_diff_vec(input: &Vec<u32>) -> Vec<u32> {
    let vec = input
        .as_slice()
        .windows(2)
        .map(|pair| pair[0].abs_diff(pair[1]))
        .collect::<Vec<_>>();
    if vec.iter().all(|x| x == 0) {
        
    }
}
