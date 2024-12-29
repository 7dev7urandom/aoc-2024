use std::collections::HashSet;

advent_of_code::solution!(10);

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect::<Vec<_>>()
}
fn breadth_search(
    grid: &Vec<Vec<u8>>,
    pos: (usize, usize),
    height: u8,
) -> (HashSet<(usize, usize)>, u32) {
    let position_height = grid[pos.0][pos.1];
    let mut hashset = HashSet::new();
    let mut sum = 0;
    if position_height != height {
        return (hashset, sum);
    }
    if position_height == 9 {
        hashset.insert(pos);
        sum += 1;
        return (hashset, sum);
    }
    if pos.0 != 0 {
        let res = breadth_search(grid, (pos.0 - 1, pos.1), position_height + 1);
        hashset.extend(res.0.iter());
        sum += res.1;
    }
    if pos.1 != 0 {
        let res = breadth_search(grid, (pos.0, pos.1 - 1), position_height + 1);
        hashset.extend(res.0.iter());
        sum += res.1;
    }
    if pos.0 != grid.len() - 1 {
        let res = breadth_search(grid, (pos.0 + 1, pos.1), position_height + 1);
        hashset.extend(res.0.iter());
        sum += res.1;
    }
    if pos.1 != grid[0].len() - 1 {
        let res = breadth_search(grid, (pos.0, pos.1 + 1), position_height + 1);
        hashset.extend(res.0.iter());
        sum += res.1;
    }
    (hashset, sum)
}
pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_input(input);
    let mut sum = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let score = breadth_search(&grid, (x, y), 0);
            sum += score.0.len() as u32;
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_input(input);
    let mut sum = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let score = breadth_search(&grid, (x, y), 0);
            sum += score.1;
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
