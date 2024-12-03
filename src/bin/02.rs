advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let numbers: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect()
        })
        .collect();
    let mut total_safe = 0;
    for number_list in numbers.iter() {
        if is_safe(number_list) {
            total_safe += 1;
        }
    }
    Some(total_safe)
}
pub fn is_safe(numbers: &Vec<i32>) -> bool {
    let mut safe = true;
    let increasing = numbers[1] > numbers[0];
    for i in 1..numbers.len() {
        if increasing {
            if numbers[i] - numbers[i - 1] > 3 || numbers[i] - numbers[i - 1] < 1 {
                safe = false;
                break;
            }
        } else {
            if numbers[i - 1] - numbers[i] > 3 || numbers[i - 1] - numbers[i] < 1 {
                safe = false;
                break;
            }
        }
    }
    safe
}
pub fn part_two(input: &str) -> Option<u32> {
    let numbers: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect()
        })
        .collect();
    let mut total_safe = 0;
    for number_list in numbers.iter() {
        let mut safe = is_safe(number_list);
        if !safe {
            for i in 0..number_list.len() {
                let mut new_list = number_list.clone();
                new_list.remove(i);
                if is_safe(&new_list) {
                    safe = true;
                    break;
                }
            }
        }
        if safe {
            total_safe += 1;
        }
    }
    Some(total_safe)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
