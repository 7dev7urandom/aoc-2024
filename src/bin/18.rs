use std::collections::HashSet;

advent_of_code::solution!(18);

#[cfg(test)]
const DIMENSIONS: usize = 6;
#[cfg(test)]
const NANOSECONDS: usize = 12;

#[cfg(not(test))]
const DIMENSIONS: usize = 70;
#[cfg(not(test))]
const NANOSECONDS: usize = 1024;

fn get_distance_to_exit(input: &str, nanoseconds: usize) -> Option<u32> {
    let block_pos = input
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|c| (c.0.parse().unwrap(), c.1.parse().unwrap()))
        .take(nanoseconds)
        .collect::<HashSet<(u32, u32)>>();
    let mut visited = HashSet::new();
    let mut queue = std::collections::VecDeque::new();
    queue.push_back((0, 0, 0));
    while let Some((x, y, steps)) = queue.pop_front() {
        if x == DIMENSIONS as u32 && y == DIMENSIONS as u32 {
            return Some(steps);
        }
        if x > DIMENSIONS as u32 || y > DIMENSIONS as u32 {
            continue;
        }
        if visited.contains(&(x, y)) {
            continue;
        }
        visited.insert((x, y));
        if !block_pos.contains(&(x, y)) {
            queue.push_back((x + 1, y, steps + 1));
            queue.push_back((x, y + 1, steps + 1));
            if x > 0 {
                queue.push_back((x - 1, y, steps + 1));
            }
            if y > 0 {
                queue.push_back((x, y - 1, steps + 1));
            }
        }
    }
    None
}
pub fn part_one(input: &str) -> Option<u32> {
    get_distance_to_exit(input, NANOSECONDS)
}

pub fn part_two(input: &str) -> Option<String> {
    let lines = input
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let mut min = 0;
    let mut max = lines.len();
    let mut i = max / 2;
    loop {
        let result = get_distance_to_exit(input, i);
        if result.is_none() {
            max = i;
            i = (min + max) / 2;
        } else {
            min = i;
            i = (min + max) / 2;
        }
        if max - min <= 1 {
            break;
        }
    }
    lines.get(i).cloned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("6,1".to_string()));
    }
}
