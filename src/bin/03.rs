advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let re = regex::Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut total = 0;
    for cap in re.captures_iter(input) {
        let a = cap[1].parse::<u32>().unwrap();
        let b = cap[2].parse::<u32>().unwrap();
        total += a * b;
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = regex::Regex::new(r"(mul\((\d+),(\d+)\)|do\(\)|don't\(\))").unwrap();
    let mut enabled = true;
    let mut total = 0;
    for cap in re.captures_iter(input) {
        if cap.get(0).unwrap().as_str() == "do()" {
            enabled = true;
        } else if cap.get(0).unwrap().as_str() == "don't()" {
            enabled = false;
        } else {
            let a = cap[2].parse::<u32>().unwrap();
            let b = cap[3].parse::<u32>().unwrap();
            if enabled {
                total += a * b;
            }
        }
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
