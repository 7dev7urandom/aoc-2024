use std::vec;

advent_of_code::solution!(1);

pub fn get_sorted_lists(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut first_numbers: Vec<u32> = vec![];
    let mut second_numbers: Vec<u32> = vec![];

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let parts = line.split("   ").collect::<Vec<&str>>();
        let first = parts[0].parse::<u32>().unwrap();
        let second = parts[1].parse::<u32>().unwrap();
        first_numbers.push(first);
        second_numbers.push(second);
    }
    first_numbers.sort();
    second_numbers.sort();
    (first_numbers, second_numbers)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (first_numbers, second_numbers) = get_sorted_lists(input);
    let sum: u32 = first_numbers
        .iter()
        .zip(second_numbers.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (first_numbers, second_numbers) = get_sorted_lists(input);
    let mut total = 0;
    for first in first_numbers.iter() {
        let count = second_numbers.iter().filter(|&x| x == first).count();
        total += count as u32 * first;
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
