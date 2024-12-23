use std::collections::{HashMap, HashSet};

advent_of_code::solution!(21);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Inst {
    Left,
    Right,
    Up,
    Down,
    DirA,
    NumA,
    Num(u32),
}
/*
+---+---+---+
| 7 | 8 | 9 |
+---+---+---+
| 4 | 5 | 6 |
+---+---+---+
| 1 | 2 | 3 |
+---+---+---+
    | 0 | A |
    +---+---+

    +---+---+
    | ^ | A |
+---+---+---+
| < | v | > |
+---+---+---+
*/
fn get_keypad_position(instruction: &Inst) -> (usize, usize) {
    match instruction {
        Inst::Left => (0, 0),
        Inst::Right => (2, 0),
        Inst::Up => (1, 1),
        Inst::Down => (1, 0),
        Inst::DirA => (2, 1),
        Inst::NumA => (2, 0),
        Inst::Num(0) => (1, 0),
        Inst::Num(z) => ((*z as usize - 1) % 3, (*z as usize - 1) / 3 + 1),
    }
}
fn routes_to_space(from: (usize, usize), to: (usize, usize), is_numpad: bool) -> Vec<Vec<Inst>> {
    let mut routes = HashSet::new();
    let x_diff = to.0 as i32 - from.0 as i32;
    let y_diff = to.1 as i32 - from.1 as i32;
    let mut x_comp = vec![];
    let mut y_comp = vec![];
    if x_diff > 0 {
        x_comp.extend(vec![Inst::Right; x_diff as usize]);
    } else if x_diff < 0 {
        x_comp.extend(vec![Inst::Left; x_diff.abs() as usize]);
    }
    if y_diff > 0 {
        y_comp.extend(vec![Inst::Up; y_diff as usize]);
    } else if y_diff < 0 {
        y_comp.extend(vec![Inst::Down; y_diff.abs() as usize]);
    }
    let mut x_then_y = x_comp.clone();
    x_then_y.extend(y_comp.clone());
    let mut y_then_x = y_comp.clone();
    y_then_x.extend(x_comp.clone());
    // Don't allow dead spaces
    if !((is_numpad && from.1 == 0 && to.0 == 0) || (!is_numpad && to.0 == 0 && from.1 == 1)) {
        // x then y is allowed
        routes.insert(x_then_y);
    }
    if !((is_numpad && to.1 == 0 && from.0 == 0) || (!is_numpad && from.0 == 0 && to.1 == 1)) {
        // y then x is allowed
        routes.insert(y_then_x);
    }
    routes
        .into_iter()
        .map(|mut v| {
            v.push(Inst::DirA);
            v
        })
        .collect()
}

fn fastest_inst_combinations(
    required_code: &Vec<Inst>,
    depth: usize,
    cache: &mut HashMap<(Vec<Inst>, usize), u64>,
) -> u64 {
    if cache.contains_key(&(required_code.clone(), depth)) {
        return *cache.get(&(required_code.clone(), depth)).unwrap();
    }
    let mut sum = 0;
    let mut current_position = get_keypad_position(match required_code[0] {
        Inst::NumA => &Inst::NumA,
        Inst::Num(_) => &Inst::NumA,
        _ => &Inst::DirA,
    });
    for inst in required_code {
        let paths = routes_to_space(
            current_position,
            get_keypad_position(inst),
            match inst {
                Inst::NumA => true,
                Inst::Num(_) => true,
                _ => false,
            },
        );
        sum += match depth {
            1 => paths[0].len() as u64,
            _ => paths
                .iter()
                .map(|p| fastest_inst_combinations(p, depth - 1, cache))
                .min()
                .unwrap(),
        };
        current_position = get_keypad_position(inst);
    }
    cache.insert((required_code.clone(), depth), sum);
    sum
}
fn run_with_depth(input: &str, depth: usize) -> Option<u64> {
    let required_codes: Vec<Vec<Inst>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    'A' => Inst::NumA,
                    _ => Inst::Num(c.to_digit(10).unwrap()),
                })
                .collect()
        })
        .collect();
    let mut cache = HashMap::new();
    let mut sum = 0;
    for code in required_codes.iter() {
        let a = fastest_inst_combinations(code, depth, &mut cache);
        let numeric = code.iter().fold(0, |acc, v| match v {
            Inst::Num(n) => acc * 10 + n,
            _ => acc,
        });
        sum += a * numeric as u64;
    }
    Some(sum)
}
pub fn part_one(input: &str) -> Option<u64> {
    run_with_depth(input, 2 + 1)
}

pub fn part_two(input: &str) -> Option<u64> {
    run_with_depth(input, 25 + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(154115708116294));
    }
}
