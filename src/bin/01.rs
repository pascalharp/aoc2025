advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let (_, hits) = input
        .lines()
        .map(|l| {
            let (dir, num) = l.split_at(1);
            let mut num = num.parse::<i32>().expect("Failed to parse number");
            if dir == "L" {
                num = -num;
            }
            num
        })
        .fold((50i32, 0u64), |(mut curr, mut hits), next| {
            curr += next;
            curr %= 100;
            if curr == 0 {
                hits += 1;
            }
            (curr, hits)
        });
    Some(hits)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, hits) = input
        .lines()
        .map(|l| {
            let (dir, num) = l.split_at(1);
            let mut num = num.parse::<i32>().expect("Failed to parse number");
            if dir == "L" {
                num = -num;
            }
            num
        })
        .fold((50i32, 0u64), |(mut curr, mut hits), next| {
            if curr == 0 && next < 0 {
                curr = 100;
            }
            curr += next;
            match curr {
                0 => hits += 1,
                c if c < 0 => hits += (c.abs() as u64 + 100) / 100,
                c if c >= 100 => hits += c as u64 / 100,
                _ => (),
            }
            curr %= 100;
            if curr < 0 {
                curr = 100 + curr
            }
            (curr, hits)
        });
    Some(hits)
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
        assert_eq!(result, Some(6));
    }
}
