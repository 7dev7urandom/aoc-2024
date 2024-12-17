use std::collections::HashSet;

advent_of_code::solution!(16);

fn parse_input(input: &str) -> ((usize, usize), (usize, usize), Vec<Vec<char>>) {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let grid = input
        .split('\n')
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| {
                    match c {
                        'S' => {
                            start = (y, x);
                        }
                        'E' => {
                            end = (y, x);
                        }
                        _ => (),
                    };
                    c
                })
                .collect()
        })
        .collect();
    (start, end, grid)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (start, end, grid) = parse_input(input);
    // Solve maze
    let score = solve_maze(&grid, start, end).unwrap();
    Some(score.0)
}
#[derive(PartialEq, Debug, Copy, Clone, Eq, Ord, PartialOrd)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Score {
    pos: (usize, usize),
    score: u32,
    direction: Direction,
}

fn insert_score(v: Score, queue: &mut std::collections::VecDeque<Score>) {
    queue.insert(
        queue
            .binary_search_by(|e| e.score.cmp(&(v.score)))
            .unwrap_or_else(|x| x),
        v,
    );
}

fn solve_maze(
    grid: &Vec<Vec<char>>,
    start: (usize, usize),
    end: (usize, usize),
) -> Option<(u32, u32)> {
    // visited: [y][x][direction] = score and spaces leading to this point with optimal score
    // visited[y][x][direction] = Option (score, [(y, x)])
    let mut visited: Vec<Vec<[Option<(u32, Vec<(usize, usize)>)>; 4]>> =
        vec![vec![[const { None }; 4]; grid[0].len()]; grid.len()];
    // solve the maze, then return the score and the number of path spaces in every optimal route
    // every turn costs 1000, every step costs 1
    // bfs
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(Score {
        pos: start,
        score: 0,
        direction: Direction::Right,
    });
    visited[start.0][start.1][Direction::Right as usize] = Some((0, vec![]));
    while let Some(Score {
        pos,
        score,
        direction,
    }) = queue.pop_front()
    {
        if pos == end {
            continue;
        }
        let new_pos = match direction {
            Direction::Up => (pos.0 - 1, pos.1),
            Direction::Down => (pos.0 + 1, pos.1),
            Direction::Left => (pos.0, pos.1 - 1),
            Direction::Right => (pos.0, pos.1 + 1),
        };
        if let Some(ref mut other) =
            visited[new_pos.0 as usize][new_pos.1 as usize][direction as usize]
        {
            if other.0 == score + 1 {
                other.1.push((pos.0, pos.1));
            } else if other.0 <= score + 1 {
                continue;
            } else {
                other.0 = score + 1;
                other.1 = vec![(pos.0, pos.1)];
            }
        } else {
            if grid[new_pos.0][new_pos.1] != '#' {
                visited[new_pos.0 as usize][new_pos.1 as usize][direction as usize] =
                    Some((score + 1, vec![(pos.0, pos.1)]));
                insert_score(
                    Score {
                        pos: new_pos,
                        score: score + 1,
                        direction: direction,
                    },
                    &mut queue,
                );
            }
        }
        for dir in [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            if dir == direction {
                continue;
            }
            let new_pos = match dir {
                Direction::Up => (pos.0 - 1, pos.1),
                Direction::Down => (pos.0 + 1, pos.1),
                Direction::Left => (pos.0, pos.1 - 1),
                Direction::Right => (pos.0, pos.1 + 1),
            };
            let original_sources = visited[pos.0 as usize][pos.1 as usize][direction as usize]
                .as_ref()
                .unwrap()
                .1
                .clone();
            if let Some(ref mut other) = visited[pos.0 as usize][pos.1 as usize][dir as usize] {
                if other.0 < score + 1000 {
                    continue;
                }
                if other.0 == score + 1000 {
                    other.1.extend(original_sources);
                    continue;
                }
                other.0 = score + 1000;
                other.1 = original_sources;
            } else {
                if grid[new_pos.0][new_pos.1] == '#' {
                    continue;
                }
                visited[pos.0 as usize][pos.1 as usize][dir as usize] = Some((
                    score + 1000,
                    visited[pos.0 as usize][pos.1 as usize][direction as usize]
                        .as_ref()
                        .unwrap()
                        .1
                        .clone(),
                ));
                insert_score(
                    Score {
                        pos: pos,
                        score: score + 1000,
                        direction: dir,
                    },
                    &mut queue,
                );
            }
        }
    }
    let best_score = visited[end.0][end.1]
        .iter()
        .filter_map(|v| v.as_ref())
        .map(|v| v.0)
        .min()
        .unwrap();
    let mut best_spaces = HashSet::new();
    best_spaces.insert(end);

    // For each sourcing space with the best score, add the spaces leading to it
    // Vector of (space, [spaces leading to it])
    let mut spaces_to_check: Vec<((usize, usize), Vec<(usize, usize)>)> = vec![(
        end,
        visited[end.0][end.1]
            .iter()
            .filter_map(|f| f.as_ref())
            .filter(|f| f.0 == best_score)
            .flat_map(|f| f.1.iter())
            .map(|f| *f)
            .collect(),
    )];
    best_spaces.extend(spaces_to_check[0].1.iter());

    while let Some((space, sources)) = spaces_to_check.pop() {
        for source in sources {
            let direction = match (
                source.0 as i32 - space.0 as i32,
                source.1 as i32 - space.1 as i32,
            ) {
                (-1, 0) => Direction::Down,
                (1, 0) => Direction::Up,
                (0, -1) => Direction::Right,
                (0, 1) => Direction::Left,
                _ => unreachable!(),
            };
            spaces_to_check.push((
                source,
                visited[source.0][source.1][direction as usize]
                    .as_ref()
                    .unwrap()
                    .1
                    .clone(),
            ));
            best_spaces.insert(source);
        }
    }

    Some((best_score, best_spaces.len() as u32))
}

pub fn part_two(input: &str) -> Option<u32> {
    let (start, end, grid) = parse_input(input);
    // Solve maze
    let score = solve_maze(&grid, start, end).unwrap();
    Some(score.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(7036));
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(45));
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(64));
    }
}
