advent_of_code::solution!(11);

// Part one is very inefficient (1.1s)- it would be only 150µs using pt 2 method.
// We keep it here as a record
pub fn part_one(input: &str) -> Option<u32> {
    let mut stones = input
        .split(' ')
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    for _ in 0..25 {
        let mut i = 0;
        loop {
            let stone = i;
            i += 1;
            if stone >= stones.len() {
                break;
            }
            if stones[stone] == 0 {
                stones[stone] = 1;
                continue;
            }
            let num_of_digits = stones[stone].ilog10() + 1;
            if num_of_digits % 2 == 0 {
                let half = num_of_digits / 2;
                let first_half = stones[stone] / 10u64.pow(half);
                let second_half = stones[stone] % 10u64.pow(half);
                stones[stone] = first_half;
                stones.insert(stone + 1, second_half);
                i += 1;
                continue;
            }
            stones[stone] *= 2024;
        }
    }
    Some(stones.len() as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut stones_totals = std::collections::HashMap::new();
    for stone in input.split(' ').map(|s| s.parse::<u64>().unwrap()) {
        *stones_totals.entry(stone).or_insert(0) += 1;
    }

    for _ in 0..75 {
        let stones = stones_totals;
        stones_totals = std::collections::HashMap::new();
        for (stone, count) in stones.iter() {
            if *stone == 0 {
                *stones_totals.entry(1).or_insert(0) += count;
            } else {
                let num_of_digits = stone.ilog10() + 1;
                if num_of_digits % 2 == 0 {
                    let half = num_of_digits / 2;
                    let first_half = stone / 10u64.pow(half);
                    let second_half = stone % 10u64.pow(half);
                    *stones_totals.entry(first_half).or_insert(0) += count;
                    *stones_totals.entry(second_half).or_insert(0) += count;
                } else {
                    *stones_totals.entry(stone * 2024).or_insert(0) += count;
                }
            }
        }
    }
    Some(stones_totals.values().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
