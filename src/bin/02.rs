advent_of_code::solution!(2);

fn is_invalid_p1(id: u64) -> bool {
    let s = id.to_string();
    if s.len() % 2 == 1 {
        false
    } else {
        let (left, right) = s.split_at(s.len() / 2);
        left == right
    }
}

fn is_invalid_p2(id: u64) -> bool {
    let s = id.to_string();
    let len = s.len();
    for i in 1..=(len / 2) {
        if !len.is_multiple_of(i) {
            continue;
        }
        let mut iter = s.as_bytes().chunks_exact(i);
        let first = iter.next().unwrap();
        if iter.all(|x| x == first) {
            return true;
        }
    }
    false
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut total = 0u64;
    for r in input.trim_end().split(',').flat_map(|s| {
        let mut split = s.split('-');
        // cursed
        split.next().unwrap().parse::<u64>().unwrap()
            ..=split.next().unwrap().parse::<u64>().unwrap()
    }) {
        if is_invalid_p1(r) {
            total += r;
        }
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut total = 0u64;
    for r in input.trim_end().split(',').flat_map(|s| {
        let mut split = s.split('-');
        // cursed
        split.next().unwrap().parse::<u64>().unwrap()
            ..=split.next().unwrap().parse::<u64>().unwrap()
    }) {
        if is_invalid_p2(r) {
            total += r;
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
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
