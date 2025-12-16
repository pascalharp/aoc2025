use std::fmt::Display;

advent_of_code::solution!(7);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Field {
    Empty,
    Beam,
    Splitter,
    Start,
}

impl From<&u8> for Field {
    fn from(value: &u8) -> Self {
        match value {
            b'.' => Self::Empty,
            b'|' => Self::Beam,
            b'^' => Self::Splitter,
            b'S' => Self::Start,
            _ => unreachable!(),
        }
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Field::Empty => write!(f, "."),
            Field::Beam => write!(f, "|"),
            Field::Splitter => write!(f, "^"),
            Field::Start => write!(f, "S"),
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let mut splits = 0u64;
    let mut last_row = lines
        .next()
        .unwrap()
        .as_bytes()
        .iter()
        .map(Field::from)
        .collect::<Vec<_>>();

    let len = last_row.len();

    for line in lines {
        let mut new_row: Vec<Field> = Vec::new();
        let mut skip = false;
        for (i, f) in line.as_bytes().iter().map(Field::from).enumerate() {
            if skip {
                skip = false;
                continue;
            }
            match (last_row[i], f) {
                (Field::Start, _) => new_row.push(Field::Beam),
                (Field::Empty, f) => new_row.push(f),
                (Field::Beam, Field::Splitter) => {
                    if i > 0 {
                        new_row[i - 1] = Field::Beam;
                    }
                    new_row.push(Field::Splitter);
                    if i < len - 1 {
                        new_row.push(Field::Beam);
                        skip = true;
                    }
                    splits += 1;
                }
                (Field::Beam, Field::Empty) => new_row.push(Field::Beam),
                (_, f) => new_row.push(f),
            }
        }
        last_row = new_row;
    }

    Some(splits)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let start = lines
        .next()
        .unwrap()
        .as_bytes()
        .iter()
        .map(Field::from)
        .collect::<Vec<_>>();

    let len = start.len();
    let mut timelines = Vec::new();
    timelines.resize(len, 1u64);

    let start = start.into_iter().position(|v| v == Field::Start).unwrap();

    for l in lines.rev() {
        for (i, f) in l.as_bytes().iter().map(Field::from).enumerate() {
            if f == Field::Splitter {
                timelines[i] = if i > 0 { timelines[i - 1] } else { 0 }
                    + if i < len - 1 { timelines[i + 1] } else { 0 };
            }
        }
    }

    Some(timelines[start])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
