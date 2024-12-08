advent_of_code::solution!(7);
// This day requires u64 instead of u32 because of rather large numbers
// Fastest way to concat two numbers. Part 2 calculates in 2.5s in release mode (almost 3x the speed of string concat)
fn concat(a: u64, b: u64) -> u64 {
    let b_digits = b.ilog10() + 1;
    a * 10u64.pow(b_digits) + b
    // (a.to_string() + &b.to_string()).parse().unwrap()
}
// Breadth-first search
fn recurse_combinations(terms: &[u64], accumulator: u64, allow_concat: bool) -> Vec<u64> {
    let mut results = vec![];
    if terms.len() == 1 {
        if allow_concat {
            return vec![
                accumulator + terms[0],
                accumulator * terms[0],
                concat(accumulator, terms[0]),
            ];
        }
        return vec![accumulator + terms[0], accumulator * terms[0]];
    }
    let adding_possibilities =
        recurse_combinations(&terms[1..], accumulator + terms[0], allow_concat);
    let multiplying_possibilities =
        recurse_combinations(&terms[1..], accumulator * terms[0], allow_concat);
    for possibility in adding_possibilities {
        results.push(possibility);
    }
    for possibility in multiplying_possibilities {
        results.push(possibility);
    }
    if allow_concat {
        let concat_possibilities =
            recurse_combinations(&terms[1..], concat(accumulator, terms[0]), allow_concat);
        for possibility in concat_possibilities {
            results.push(possibility);
        }
    }
    results
}
pub fn part_one(input: &str) -> Option<u64> {
    let lines: Vec<(u64, Vec<u64>)> = input
        .lines()
        .map(|l| {
            let (result, terms) = l.split_once(": ").unwrap();
            let result: u64 = result.parse().unwrap();
            let terms = terms.split(" ").map(|t| t.parse().unwrap()).collect();
            (result, terms)
        })
        .collect();
    let mut total_valid: u64 = 0;
    for (result, terms) in lines {
        let possibilities = recurse_combinations(&terms, 0, false);
        if possibilities.contains(&(result)) {
            total_valid += result;
        }
    }
    Some(total_valid)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines: Vec<(u64, Vec<u64>)> = input
        .lines()
        .map(|l| {
            let (result, terms) = l.split_once(": ").unwrap();
            let result: u64 = result.parse().unwrap();
            let terms = terms.split(" ").map(|t| t.parse().unwrap()).collect();
            (result, terms)
        })
        .collect();
    let mut total_valid: u64 = 0;
    for (result, terms) in lines {
        let possibilities = recurse_combinations(&terms, 0, true);
        if possibilities.contains(&(result)) {
            total_valid += result;
        }
    }
    Some(total_valid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
