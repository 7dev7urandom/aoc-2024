use itertools::Itertools;

advent_of_code::solution!(13);
fn parse_input(input: &str, add_ten_trillion: bool) -> Vec<((u64, u64), (u64, u64), (u64, u64))> {
    input
        .split("\n\n")
        .map(|section| {
            let mut lines = section.lines();
            let button_a_text = lines.next().unwrap().split(": ").nth(1).unwrap();
            let button_b_text = lines.next().unwrap().split(": ").nth(1).unwrap();
            let prize_text = lines.next().unwrap().split(": ").nth(1).unwrap();
            let (ax, ay) = button_a_text
                .split(", ")
                .map(|s| s[2..].parse().unwrap())
                .collect_tuple()
                .unwrap();
            let (bx, by) = button_b_text
                .split(", ")
                .map(|s| s[2..].parse().unwrap())
                .collect_tuple()
                .unwrap();
            let (px, py) = prize_text
                .split(", ")
                .map(|s| {
                    s[2..].parse::<u64>().unwrap()
                        + if add_ten_trillion {
                            10000000000000u64
                        } else {
                            0
                        }
                })
                .collect_tuple()
                .unwrap();
            ((ax, ay), (bx, by), (px, py))
        })
        .collect()
}

fn solve_equation_system(
    (ax, ay): (u64, u64),
    (bx, by): (u64, u64),
    (px, py): (u64, u64),
) -> Option<(u64, u64)> {
    // (ay*px - py*ax) / (ay*bx - by*ax) = B
    let b = ((ay * px).abs_diff(py * ax)) / ((ay * bx).abs_diff(by * ax));
    // (px - bx*B) / ax = A
    let a = (px.abs_diff(bx * b)) / ax;

    if ax * a + bx * b == px && ay * a + by * b == py {
        Some((a, b))
    } else {
        None
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let machines = parse_input(input, false);
    let mut sum = 0;
    for machine in machines {
        let res = solve_equation_system(machine.0, machine.1, machine.2);
        if let Some((a, b)) = res {
            sum += a * 3 + b;
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let machines = parse_input(input, true);
    let mut sum = 0;
    for machine in machines {
        let res = solve_equation_system(machine.0, machine.1, machine.2);
        if let Some((a, b)) = res {
            sum += a * 3 + b;
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
