use std::{
    collections::{HashMap, VecDeque},
    vec,
};

advent_of_code::solution!(20);
fn parse_input(input: &str) -> ((usize, usize), (usize, usize), Vec<Vec<char>>) {
    let mut start_pos = (0, 0);
    let mut end_pos = (0, 0);
    let grid = input
        .lines()
        .enumerate()
        .map(|(y, f)| {
            f.chars()
                .enumerate()
                .map(|(x, c)| {
                    match c {
                        'S' => start_pos = (x, y),
                        'E' => end_pos = (x, y),
                        _ => (),
                    }
                    c
                })
                .collect()
        })
        .collect();
    (start_pos, end_pos, grid)
}
fn create_backwards_rank(position: (usize, usize), grid: &Vec<Vec<char>>) -> Vec<Vec<Option<u32>>> {
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut queue = VecDeque::new();
    queue.push_back((position, 0));
    let mut steps_vec = vec![vec![None; grid[0].len()]; grid.len()];
    while !queue.is_empty() {
        let (pos, steps) = queue.pop_front().unwrap();
        if visited[pos.1][pos.0] {
            continue;
        }
        steps_vec[pos.1][pos.0] = Some(steps);
        visited[pos.1][pos.0] = true;
        if grid[pos.1][pos.0] == 'S' {
            return steps_vec;
        }
        queue.push_back((get_next(grid, pos, &visited), steps + 1));
    }
    steps_vec
}
fn get_next(
    grid: &Vec<Vec<char>>,
    pos: (usize, usize),
    visited: &Vec<Vec<bool>>,
) -> (usize, usize) {
    if grid[pos.1 + 1][pos.0] != '#' && !visited[pos.1 + 1][pos.0] {
        return (pos.0, pos.1 + 1);
    }
    if grid[pos.1 - 1][pos.0] != '#' && !visited[pos.1 - 1][pos.0] {
        return (pos.0, pos.1 - 1);
    }
    if grid[pos.1][pos.0 + 1] != '#' && !visited[pos.1][pos.0 + 1] {
        return (pos.0 + 1, pos.1);
    }
    if grid[pos.1][pos.0 - 1] != '#' && !visited[pos.1][pos.0 - 1] {
        return (pos.0 - 1, pos.1);
    }
    unreachable!()
}
fn get_cheats_with_minimum_benefit(
    current_pos: (usize, usize),
    grid: &Vec<Vec<char>>,
    no_cheat_times: &Vec<Vec<Option<u32>>>,
    min_improvement: u32,
    allowed_cheat_length: i32,
) -> HashMap<u32, u32> {
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut current_pos = current_pos;
    let mut saved = HashMap::new();
    loop {
        for dy in (-allowed_cheat_length)..=allowed_cheat_length {
            for dx in (-(allowed_cheat_length - dy.abs()))..=(allowed_cheat_length - dy.abs()) {
                if current_pos.0 as i32 + dx < 0 || current_pos.1 as i32 + dy < 0 {
                    continue;
                }
                let (x, y) = (
                    (current_pos.0 as i32 + dx) as usize,
                    (current_pos.1 as i32 + dy) as usize,
                );
                if x < grid[0].len() && y < grid.len() {
                    if grid[y][x] != '#' {
                        if let Some(target_time) = no_cheat_times[y][x] {
                            let total = dx.abs() + dy.abs() + target_time as i32;
                            let total_saved = no_cheat_times[current_pos.1][current_pos.0].unwrap()
                                as i32
                                - total;
                            if total_saved >= min_improvement as i32 {
                                *saved.entry(total_saved as u32).or_insert(0u32) += 1;
                            }
                        }
                    }
                }
            }
        }
        visited[current_pos.1][current_pos.0] = true;
        current_pos = get_next(&grid, current_pos, &visited);
        if grid[current_pos.1][current_pos.0] == 'E' {
            break;
        }
    }
    saved
}
pub fn part_one(input: &str) -> Option<u32> {
    let (start_pos, end_pos, grid) = parse_input(input);
    let no_cheat_times = create_backwards_rank(end_pos, &grid);
    #[cfg(test)]
    const MIN_IMPROVEMENT: u32 = 10;
    #[cfg(not(test))]
    const MIN_IMPROVEMENT: u32 = 100;
    let saved =
        get_cheats_with_minimum_benefit(start_pos, &grid, &no_cheat_times, MIN_IMPROVEMENT, 2);
    Some(saved.values().sum())
}
pub fn part_two(input: &str) -> Option<u32> {
    let (start_pos, end_pos, grid) = parse_input(input);
    let no_cheat_times = create_backwards_rank(end_pos, &grid);
    #[cfg(test)]
    const MIN_IMPROVEMENT: u32 = 50;
    #[cfg(not(test))]
    const MIN_IMPROVEMENT: u32 = 100;
    let saved =
        get_cheats_with_minimum_benefit(start_pos, &grid, &no_cheat_times, MIN_IMPROVEMENT, 20);
    Some(saved.values().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(285));
    }
}
