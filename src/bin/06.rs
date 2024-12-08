use colored::Colorize;

advent_of_code::solution!(6);
#[derive(Debug, Clone, Copy, PartialEq)]
enum Position {
    Obstacle,
    Empty,
    Traversed((i32, i32)),
}
fn parse_input_to_grid(input: &str) -> ((usize, usize), Vec<Vec<Position>>) {
    let mut start_position = (0, 0);
    let positions = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '#' => Position::Obstacle,
                    '.' => Position::Empty,
                    '^' => {
                        start_position = (x, y);
                        Position::Traversed((0, -1))
                    }
                    _ => panic!("Invalid character in input"),
                })
                .collect()
        })
        .collect();
    return (start_position, positions);
}
pub fn part_one(input: &str) -> Option<u32> {
    let (mut current_position, mut grid) = parse_input_to_grid(input);
    const DIRECTIONS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut current_direction = 0;
    loop {
        // println!("{}, {}", current_position.0, current_position.1);
        let (x, y) = current_position;
        let (dx, dy) = DIRECTIONS[current_direction];
        let (nx, ny) = (x as i32 + dx, y as i32 + dy);
        if nx < 0 || ny < 0 {
            break;
        }
        let (nx, ny) = (nx as usize, ny as usize);
        if nx >= grid[0].len() || ny >= grid.len() {
            break;
        }
        if grid[ny][nx] == Position::Obstacle {
            current_direction = (current_direction + 1) % 4;
            continue;
        }
        grid[y][x] = Position::Traversed((dx, dy));
        current_position = (nx, ny);
    }
    Some(
        grid.iter()
            .flatten()
            .filter(|&&x| matches!(x, Position::Traversed { .. }))
            .count() as u32
            + 1,
    )
}
const DIRECTIONS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
#[allow(dead_code)]
fn print_grid(
    grid: &Vec<Vec<Position>>,
    position: (usize, usize),
    marked_positions: Vec<(usize, usize)>,
) {
    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if (x, y) == position {
                print!("{}", "X".red());
                continue;
            }
            let chr = match cell {
                Position::Obstacle => "#",
                Position::Empty => ".",
                Position::Traversed((0, -1)) => "^",
                Position::Traversed((1, 0)) => ">",
                Position::Traversed((0, 1)) => "v",
                Position::Traversed((-1, 0)) => "<",
                Position::Traversed(_) => "*",
            };
            if marked_positions.contains(&(x, y)) {
                print!("{}", chr.green());
            } else {
                print!("{}", chr);
            }
        }
        println!();
    }
}
fn check_loops(
    grid: &mut Vec<Vec<Position>>,
    position: (usize, usize),
    mut direction: usize,
) -> bool {
    let (mut x, mut y) = position;
    let (mut dx, mut dy) = DIRECTIONS[direction];
    loop {
        let (nx, ny) = (x as i32 + dx, y as i32 + dy);
        if nx < 0 || ny < 0 {
            return false;
        }
        let (nx, ny) = (nx as usize, ny as usize);
        if nx >= grid[0].len() || ny >= grid.len() {
            return false;
        }
        if grid[ny][nx] == Position::Traversed((dx, dy)) {
            return true;
        }
        if grid[ny][nx] == Position::Obstacle {
            direction = (direction + 1) % 4;
            // Handle annoying 2-length loops
            if let Position::Traversed(..) = grid[y][x] {
            } else {
                grid[y][x] = Position::Traversed((dx, dy));
            }
            (dx, dy) = DIRECTIONS[direction];
            continue;
        }
        // Move forward
        if let Position::Traversed(..) = grid[y][x] {
        } else {
            grid[y][x] = Position::Traversed((dx, dy));
        }
        x = nx;
        y = ny;
    }
}
pub fn part_two(input: &str) -> Option<u32> {
    let (mut current_position, mut grid) = parse_input_to_grid(input);
    let mut current_direction = 0;
    let mut new_obstacle_locations: Vec<(usize, usize)> = vec![];
    loop {
        let (x, y) = current_position;
        let (dx, dy) = DIRECTIONS[current_direction];
        let (nx, ny) = (x as i32 + dx, y as i32 + dy);
        if nx < 0 || ny < 0 {
            break;
        }
        let (nx, ny) = (nx as usize, ny as usize);
        if nx >= grid[0].len() || ny >= grid.len() {
            break;
        }
        if grid[ny][nx] == Position::Empty {
            let mut copy = grid.clone();
            copy[ny][nx] = Position::Obstacle;
            if check_loops(&mut copy, (x, y), current_direction) {
                new_obstacle_locations.push((nx, ny));
            }
        }

        if grid[ny][nx] == Position::Obstacle {
            current_direction = (current_direction + 1) % 4;
            continue;
        }
        if let Position::Traversed(..) = grid[y][x] {
        } else {
            grid[y][x] = Position::Traversed((dx, dy));
        }
        current_position = (nx, ny);
    }
    Some(new_obstacle_locations.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
