advent_of_code::solution!(3);

fn find_first_max(input: &str) -> (usize, u64) {
    let (val, idx) =
        input
            .chars()
            .enumerate()
            .fold((0u32, 0usize), |(max, max_idx), (idx, val)| {
                let v = val.to_digit(10).unwrap();
                if v > max { (v, idx) } else { (max, max_idx) }
            });
    (idx, val as u64)
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut total = 0u64;
    for l in input.lines() {
        let (tenth_idx, tenth) = find_first_max(&l[0..l.len() - 1]);
        let (_, ones) = find_first_max(&l[tenth_idx + 1..]);
        total += tenth as u64 * 10 + ones as u64;
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut total = 0u64;
    for l in input.lines() {
        let mut start_idx = 0;
        for exp in (0..=11).rev() {
            let (idx, val) = find_first_max(&l[start_idx..l.len() - exp]);
            total += val * 10u64.pow(exp as u32);
            start_idx += idx + 1;
        }
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
