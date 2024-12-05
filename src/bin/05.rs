advent_of_code::solution!(5);

pub fn parse_input(input: &str) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let (part_1, part_2) = input.split_once("\n\n").unwrap();
    (
        part_1
            .lines()
            .map(|line| {
                let (row, col) = line.split_once("|").unwrap();
                (row.parse().unwrap(), col.parse().unwrap())
            })
            .collect(),
        part_2
            .lines()
            .map(|line| line.split(",").map(|num| num.parse().unwrap()).collect())
            .collect(),
    )
}
pub fn mapify_rules(rules: &Vec<(u32, u32)>) -> std::collections::HashMap<u32, Vec<u32>> {
    let mut map = std::collections::HashMap::new();
    for (required_page, printed_page) in rules.iter() {
        let entry = map.entry(*printed_page).or_insert(vec![]);
        entry.push(*required_page);
    }
    map
}
pub fn part_one(input: &str) -> Option<u32> {
    let (page_rules, print_lists) = parse_input(input);
    let page_rules = mapify_rules(&page_rules);
    let mut total = 0;
    for print in print_lists.iter() {
        let mut valid = true;
        for (printed_page, required_pages) in page_rules.iter() {
            let printed_index = print.iter().position(|&x| x == *printed_page);
            let required_indicies = required_pages
                .iter()
                .map(|page| print.iter().position(|&x| x == *page))
                .collect::<Vec<Option<usize>>>();
            if printed_index.is_none() {
                continue;
            }
            for required_index in required_indicies.iter() {
                if required_index.is_none() {
                    continue;
                }
                if required_index.unwrap() > printed_index.unwrap() {
                    valid = false;
                    break;
                }
            }
        }
        if valid {
            total += print[print.len() / 2];
        }
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (page_rules, mut print_lists) = parse_input(input);
    let page_rules = mapify_rules(&page_rules);
    let mut total = 0;
    for print in print_lists.iter_mut() {
        let mut originally_valid = true;
        // Verify the print order, fixing it if necessary
        // Repeat until the print order is valid
        loop {
            let mut no_fixes = true;
            for (printed_page, required_pages) in page_rules.iter() {
                let printed_index = print.iter().position(|&x| x == *printed_page);
                let required_indicies = required_pages
                    .iter()
                    .map(|page| print.iter().position(|&x| x == *page))
                    .collect::<Vec<Option<usize>>>();
                if printed_index.is_none() {
                    continue;
                }
                for required_index in required_indicies.iter() {
                    if required_index.is_none() {
                        continue;
                    }
                    if required_index.unwrap() > printed_index.unwrap() {
                        originally_valid = false;
                        no_fixes = false;
                        let required_page = print[required_index.unwrap()];
                        print.remove(required_index.unwrap());
                        print.insert(printed_index.unwrap(), required_page);
                        break;
                    }
                }
            }
            if no_fixes {
                break;
            }
        }
        if !originally_valid {
            total += print[print.len() / 2];
        }
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
