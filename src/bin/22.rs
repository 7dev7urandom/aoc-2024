use std::collections::HashMap;

advent_of_code::solution!(22);
fn full_lists_from_initial_price(initial_secret: Vec<u64>) -> Vec<Vec<u64>> {
    let mut buyer_pseudorandom_lists = initial_secret
        .iter()
        .map(|x| vec![*x])
        .collect::<Vec<Vec<u64>>>();

    for _ in 0..2000 {
        for buyer_list in buyer_pseudorandom_lists.iter_mut() {
            let mut next_number = buyer_list[buyer_list.len() - 1];
            next_number = (next_number ^ (next_number * 64)) % 16777216;
            next_number = (next_number ^ (next_number / 32)) % 16777216;
            next_number = (next_number ^ (next_number * 2048)) % 16777216;
            buyer_list.push(next_number);
        }
    }
    buyer_pseudorandom_lists
}
pub fn part_one(input: &str) -> Option<u64> {
    let buyer_initial = input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    let buyer_lists = full_lists_from_initial_price(buyer_initial);
    Some(buyer_lists.iter().map(|v| *v.last().unwrap()).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let buyer_initial = input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    let buyer_prices: Vec<Vec<u64>> = full_lists_from_initial_price(buyer_initial)
        .iter()
        .map(|prices| prices.iter().map(|price| price % 10).collect::<Vec<u64>>())
        .collect();
    let buyer_price_differences: Vec<Vec<i64>> = buyer_prices
        .iter()
        .map(|prices| {
            prices
                .windows(2)
                .map(|pair| pair[1] as i64 - pair[0] as i64)
                .collect()
        })
        .collect();
    let mut all_combos = HashMap::new();
    buyer_price_differences
        .iter()
        .enumerate()
        .for_each(|(w, x)| {
            x.windows(4).enumerate().for_each(|(i, y)| {
                // window 2 and window 4; get the value from the corresponding last index of the buyer_prices
                let m = buyer_prices[w][i + 1 + 3];
                let v = all_combos.entry(y).or_insert(HashMap::new());
                v.entry(w).or_insert(m);
            });
        });
    let max = all_combos
        .values()
        .map(|v| v.values().sum::<u64>())
        .max()
        .unwrap();
    Some(max as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(23));
    }
}
