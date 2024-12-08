use std::collections::{hash_map, hash_set};

advent_of_code::solution!(8);

fn parse_input(input: &str) -> (usize, usize, hash_map::HashMap<char, Vec<(usize, usize)>>) {
    // Need total width, height, and position of each non-. character
    // Each character should be organized by what char it is
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();
    let mut chars = hash_map::HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                chars.entry(c).or_insert(Vec::new()).push((x, y));
            }
        }
    }
    (width, height, chars)
}
pub fn part_one(input: &str) -> Option<u32> {
    let (width, height, chars) = parse_input(input);
    let mut antinodes = hash_set::HashSet::new();
    for char_list in chars.values() {
        for a in 0..char_list.len() {
            for b in a + 1..char_list.len() {
                let (dx, dy) = (
                    char_list[a].0 as i32 - char_list[b].0 as i32,
                    char_list[a].1 as i32 - char_list[b].1 as i32,
                );
                let first = (char_list[a].0 as i32 + dx, char_list[a].1 as i32 + dy);
                let second = (char_list[b].0 as i32 - dx, char_list[b].1 as i32 - dy);
                if first.0 >= 0 && first.0 < width as i32 && first.1 >= 0 && first.1 < height as i32
                {
                    antinodes.insert(first);
                }
                if second.0 >= 0
                    && second.0 < width as i32
                    && second.1 >= 0
                    && second.1 < height as i32
                {
                    antinodes.insert(second);
                }
            }
        }
    }
    Some(antinodes.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (width, height, chars) = parse_input(input);
    let mut antinodes = hash_set::HashSet::new();
    for char_list in chars.values() {
        for a in 0..char_list.len() {
            for b in a + 1..char_list.len() {
                let (dx, dy) = (
                    char_list[a].0 as i32 - char_list[b].0 as i32,
                    char_list[a].1 as i32 - char_list[b].1 as i32,
                );
                antinodes.insert((char_list[a].0 as i32, char_list[a].1 as i32));
                antinodes.insert((char_list[b].0 as i32, char_list[b].1 as i32));
                let mut first = (char_list[a].0 as i32 + dx, char_list[a].1 as i32 + dy);
                while first.0 >= 0
                    && first.0 < width as i32
                    && first.1 >= 0
                    && first.1 < height as i32
                {
                    antinodes.insert(first);
                    first = (first.0 + dx, first.1 + dy);
                }
                let mut second = (char_list[b].0 as i32 - dx, char_list[b].1 as i32 - dy);
                while second.0 >= 0
                    && second.0 < width as i32
                    && second.1 >= 0
                    && second.1 < height as i32
                {
                    antinodes.insert(second);
                    second = (second.0 - dx, second.1 - dy);
                }
            }
        }
    }
    Some(antinodes.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
