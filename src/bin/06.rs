advent_of_code::solution!(6);

#[derive(Debug, PartialEq, Eq)]
enum Instructions {
    Add,
    Mul,
    Empty,
}

pub fn part_one(input: &str) -> Option<u64> {
    let ins = input
        .lines()
        .last()
        .unwrap()
        .split_whitespace()
        .map(|i| match i {
            "+" => Instructions::Add,
            "*" => Instructions::Mul,
            x => panic!("Invalid Instruction {}", x),
        })
        .collect::<Vec<_>>();
    let len = input.lines().count() - 1;
    let vals = input
        .lines()
        .take(len)
        .map(|l| {
            l.split_whitespace()
                .map(|d| d.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut total = 0;
    for (c, i) in ins.iter().enumerate() {
        let mut acc = 0;
        let f = match i {
            Instructions::Add => std::ops::Add::add,
            Instructions::Mul => {
                acc += 1;
                std::ops::Mul::mul
            }
            _ => panic!("Not in this part"),
        };

        for val in vals.iter().take(len) {
            acc = f(acc, val[c])
        }
        total += acc;
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let ins = input
        .lines()
        .last()
        .unwrap()
        .chars()
        .map(|i| match i {
            '+' => Instructions::Add,
            '*' => Instructions::Mul,
            ' ' => Instructions::Empty,
            x => panic!("Invalid Instruction {}", x),
        })
        .collect::<Vec<_>>();

    let col_count = ins.len();

    let mut nums: Vec<Vec<u8>> = Vec::new();
    for _ in 0..col_count {
        nums.push(Vec::new());
    }

    for row in input.lines().collect::<Vec<_>>().iter().rev().skip(1) {
        for col_idx in 0..col_count {
            if let Some(c) = row.as_bytes().get(col_idx) {
                match c {
                    b' ' => (),
                    k if k.is_ascii_digit() => nums.get_mut(col_idx).unwrap().push(k - b'0'),
                    c => panic!("Should not occure {}", c),
                }
            }
        }
    }

    let nums = nums
        .into_iter()
        .map(|v| {
            v.into_iter()
                .enumerate()
                .fold(0, |acc, (idx, v)| acc + v as u64 * 10u64.pow(idx as u32))
        })
        .collect::<Vec<_>>();

    let mut curr = ins.first().unwrap();
    let mut acc = 0u64;
    let mut inner_acc: u64 = 0;

    for (i, ins) in ins.iter().enumerate() {
        if *ins != Instructions::Empty {
            acc += inner_acc;
            match ins {
                Instructions::Add => inner_acc = 0,
                Instructions::Mul => inner_acc = 1,
                Instructions::Empty => unreachable!(),
            }
            curr = ins;
        }
        match curr {
            Instructions::Add => inner_acc += nums[i],
            Instructions::Mul => {
                if nums[i] > 0 {
                    inner_acc *= nums[i]
                }
            }
            Instructions::Empty => unreachable!(),
        }
    }

    acc += inner_acc;
    Some(acc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
