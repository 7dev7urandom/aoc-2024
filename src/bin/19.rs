use std::collections::HashMap;

advent_of_code::solution!(19);

fn parse_input(input: &str) -> (Vec<String>, Vec<String>) {
    let (part_1, part_2) = input.split_once("\n\n").unwrap();
    (
        part_1.split(", ").map(|f| f.chars().collect()).collect(),
        part_2.lines().map(|f| f.chars().collect()).collect(),
    )
}
fn recurse_towel_arrangements(
    target_pattern: &str,
    available_towels: &Vec<String>,
    ways_to_make_pattern: &mut HashMap<String, u64>,
) -> u64 {
    if target_pattern == "" {
        return 1;
    }
    if ways_to_make_pattern.contains_key(target_pattern) {
        return ways_to_make_pattern[target_pattern];
    }
    let mut total_arrangements = 0;
    for towel in available_towels {
        if target_pattern.starts_with(towel) {
            total_arrangements += recurse_towel_arrangements(
                &target_pattern[towel.len()..],
                available_towels,
                ways_to_make_pattern,
            );
        }
    }
    ways_to_make_pattern.insert(target_pattern.to_string(), total_arrangements);
    total_arrangements
}
pub fn part_one(input: &str) -> Option<u32> {
    let (available_towels, required_patterns) = parse_input(input);
    let mut total_possible = 0;
    for required_pattern in required_patterns {
        if recurse_towel_arrangements(&required_pattern, &available_towels, &mut HashMap::new()) > 0
        {
            total_possible += 1;
        }
    }
    Some(total_possible)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (available_towels, required_patterns) = parse_input(input);
    let mut total_possible = 0;
    for required_pattern in required_patterns {
        total_possible +=
            recurse_towel_arrangements(&required_pattern, &available_towels, &mut HashMap::new());
    }
    Some(total_possible)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
