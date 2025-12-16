advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let (ranges, ids) = {
        let mut iter = input.split("\n\n");
        (iter.next().unwrap(), iter.next().unwrap())
    };

    let ranges = ranges
        .lines()
        .map(|l| {
            let mut iter = l.split("-");
            (iter.next().unwrap(), iter.next().unwrap())
        })
        .map(|(left, right)| left.parse::<u64>().unwrap()..=right.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mut count = 0;
    for id in ids.lines().map(|l| l.parse::<u64>().unwrap()) {
        if ranges.iter().any(|r| r.contains(&id)) {
            count += 1;
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut ranges = input
        .split("\n\n")
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            let mut iter = l.split("-");
            (iter.next().unwrap(), iter.next().unwrap())
        })
        .map(|(left, right)| left.parse::<u64>().unwrap()..=right.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    ranges.sort_by(|a, b| a.start().cmp(b.start()));
    let (count, _) = ranges.iter().fold((0usize, 0u64), |(count, start), range| {
        let s = start.max(*range.start());
        let e = *range.end();
        let count = count + (s..=e).count();
        (count, (start).max(e + 1))
    });
    Some(count as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
