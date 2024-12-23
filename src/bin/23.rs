use std::collections::{HashMap, HashSet};

use itertools::Itertools;

advent_of_code::solution!(23);

fn parse_input(input: &str) -> HashMap<[char; 2], HashSet<[char; 2]>> {
    let mut map = HashMap::new();
    input.lines().for_each(|line| {
        let (first, second) = line.split_once("-").unwrap();
        let first = [first.chars().nth(0).unwrap(), first.chars().nth(1).unwrap()];
        let second = [
            second.chars().nth(0).unwrap(),
            second.chars().nth(1).unwrap(),
        ];
        map.entry(first).or_insert(HashSet::new()).insert(second);
        map.entry(second).or_insert(HashSet::new()).insert(first);
    });
    map
}
pub fn part_one(input: &str) -> Option<u32> {
    let map = parse_input(input);
    let mut triple_links = HashSet::new();
    for e in map.iter().filter(|e| e.0[0] == 't') {
        for e2 in e.1.iter() {
            for e3 in map.get(e2).unwrap().iter() {
                if map.get(e3).unwrap().contains(e.0) {
                    let mut triplet = [e.0, e2, e3];
                    triplet.sort();
                    triple_links.insert(triplet);
                }
            }
        }
    }
    Some(triple_links.len() as u32)
}
pub fn part_two(input: &str) -> Option<String> {
    let map = parse_input(input);
    let mut all_cliques: Vec<HashSet<&[char; 2]>> = vec![];
    for node in map.iter() {
        let mut cliques_to_add = vec![];
        for set in all_cliques.iter() {
            if set.iter().all(|e| node.1.contains(*e)) {
                let mut new_clique = set.clone();
                new_clique.insert(node.0);
                cliques_to_add.push(new_clique);
            }
        }
        let mut fundamental_clique = HashSet::new();
        fundamental_clique.insert(node.0);
        cliques_to_add.push(fundamental_clique);
        all_cliques.extend(cliques_to_add);
    }
    let largest_clique = all_cliques.iter().max_by_key(|e| e.len()).unwrap();
    Some(
        largest_clique
            .iter()
            .map(|e| e.iter().collect::<String>())
            .sorted()
            .join(","),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("co,de,ka,ta".to_string()));
    }
}
