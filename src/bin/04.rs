advent_of_code::solution!(4);

fn is_occupied<T: AsRef<[u8]>>(mat: &[T], row: i32, col: i32) -> bool {
    if row < 0 || col < 0 {
        return false;
    }
    let row = row as usize;
    let col = col as usize;
    if let Some(row) = mat.get(row)
        && let Some(val) = row.as_ref().get(col)
            && *val == b'@' {
                return true;
            }
    false
}

fn is_reachable<T: AsRef<[u8]>>(mat: &[T], row: i32, col: i32) -> bool {
    let surroundings: [(i32, i32); 8] = [
        (row - 1, col - 1),
        (row - 1, col),
        (row - 1, col + 1),
        (row, col - 1),
        (row, col + 1),
        (row + 1, col - 1),
        (row + 1, col),
        (row + 1, col + 1),
    ];

    surroundings
        .into_iter()
        .map(|(row, col)| is_occupied(mat, row, col))
        .filter(|v| *v)
        .count() < 4
}

fn check_unreachble<T: AsRef<[u8]>>(mat: &[T]) -> impl Iterator<Item = (usize, usize)> {
    mat.iter().enumerate().flat_map(move |(row_idx, row)| {
        row.as_ref()
            .iter()
            .enumerate()
            .filter_map(move |(col_idx, val)| {
                if *val != b'@' {
                    None
                } else if is_reachable(mat, row_idx as i32, col_idx as i32) {
                    Some((row_idx, col_idx))
                } else {
                    None
                }
            })
    })
}

pub fn part_one(input: &str) -> Option<u64> {
    let mat = input.lines().map(|l| l.as_bytes()).collect::<Vec<_>>();
    let count = check_unreachble(&mat).count();
    Some(count as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut input = input.as_bytes().to_vec();

    let mut mat = input.split_mut(|v| *v == b'\n').collect::<Vec<_>>();
    let mut count: usize = 0;
    loop {
        let picked_up = check_unreachble(mat.as_slice()).collect::<Vec<_>>();
        match picked_up.len() {
            0 => break,
            val => count += val,
        };
        for (row, col) in picked_up {
            *mat.get_mut(row).unwrap().get_mut(col).unwrap() = b'.';
        }
    }

    Some(count as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
