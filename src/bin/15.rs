advent_of_code::solution!(15);

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Blank,
    Box(bool),
    Wall,
}
fn parse_input(
    input: &str,
    mapper: &dyn Fn(char, (usize, usize)) -> (Vec<Tile>, Option<(usize, usize)>),
) -> (Vec<Vec<Tile>>, Vec<char>, (usize, usize)) {
    let (sec1, sec2) = input.split_once("\n\n").unwrap();
    let mut pos = (0, 0);
    (
        sec1.split("\n")
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        _ => {
                            let res = mapper(c, (y, x));
                            if let Some(p) = res.1 {
                                pos = p;
                            }
                            res.0
                        }
                    })
                    .flatten()
                    .collect()
            })
            .collect(),
        sec2.chars().filter(|c| *c != '\n').collect(),
        pos,
    )
}
pub fn part_one(input: &str) -> Option<u32> {
    let (mut map, commands, mut robot_pos) = parse_input(input, &|c, p| match c {
        '.' => (vec![Tile::Blank], None),
        '#' => (vec![Tile::Wall], None),
        '@' => (vec![Tile::Blank], Some(p)),
        'O' => (vec![Tile::Box(false)], None),
        _ => panic!(),
    });
    for command in commands {
        let delta = match command {
            'v' => (1, 0),
            '<' => (0, -1),
            '>' => (0, 1),
            '^' => (-1, 0),
            _ => panic!(),
        };
        let mut check_pos = robot_pos;
        let mut n = 0;
        loop {
            check_pos = (
                check_pos.0.wrapping_add_signed(delta.0),
                check_pos.1.wrapping_add_signed(delta.1),
            );
            n += 1;
            match map[check_pos.0][check_pos.1] {
                Tile::Blank => {
                    if n == 1 {
                        robot_pos = check_pos;
                        break;
                    }
                    robot_pos = (
                        robot_pos.0.wrapping_add_signed(delta.0),
                        robot_pos.1.wrapping_add_signed(delta.1),
                    );
                    map[robot_pos.0][robot_pos.1] = Tile::Blank;
                    map[check_pos.0][check_pos.1] = Tile::Box(true);
                    break;
                }
                Tile::Box(_) => {}
                Tile::Wall => break,
            }
        }
    }
    Some(
        map.iter()
            .enumerate()
            .map(|(y, r)| {
                r.iter()
                    .enumerate()
                    .map(move |(x, c)| if let Tile::Box(_) = c { 100 * y + x } else { 0 })
            })
            .flatten()
            .sum::<usize>() as u32,
    )
}
fn can_push(map: &Vec<Vec<Tile>>, pos: (usize, usize), delta: (isize, isize)) -> bool {
    let check_pos = (
        pos.0.wrapping_add_signed(delta.0),
        pos.1.wrapping_add_signed(delta.1),
    );
    match map[check_pos.0][check_pos.1] {
        Tile::Blank => true,
        Tile::Box(left) => {
            if delta.0 == 0 {
                // moving left or right
                return can_push(map, check_pos, delta);
            }
            let other = if left {
                (check_pos.0, check_pos.1 + 1)
            } else {
                (check_pos.0, check_pos.1 - 1)
            };
            can_push(map, check_pos, delta) && can_push(map, other, delta)
        }
        Tile::Wall => false,
    }
}
fn move_delta(
    map: &mut Vec<Vec<Tile>>,
    pos: (usize, usize),
    delta: (isize, isize),
) -> (usize, usize) {
    let mut check_pos = pos;
    check_pos = (
        check_pos.0.wrapping_add_signed(delta.0),
        check_pos.1.wrapping_add_signed(delta.1),
    );
    // println!("{:?}", check_pos);
    match map[check_pos.0][check_pos.1] {
        Tile::Blank => {
            map[check_pos.0][check_pos.1] = map[pos.0][pos.1];
        }
        Tile::Box(left) => {
            if delta.0 == 0 {
                // moving left or right
                move_delta(map, check_pos, delta);
                map[check_pos.0][check_pos.1] = map[pos.0][pos.1];
                return check_pos;
            }
            let other = if left {
                (check_pos.0, check_pos.1 + 1)
            } else {
                (check_pos.0, check_pos.1 - 1)
            };
            move_delta(map, check_pos, delta);
            match map[pos.0][pos.1] {
                Tile::Box(current_left) => {
                    if current_left != left {
                        move_delta(map, other, delta);
                        map[other.0][other.1] = Tile::Blank;
                    }
                }
                Tile::Blank => {
                    move_delta(map, other, delta);
                    map[other.0][other.1] = Tile::Blank;
                }
                _ => {}
            }
            map[check_pos.0][check_pos.1] = map[pos.0][pos.1];
        }
        Tile::Wall => panic!("Can't move into wall at {:?}", check_pos),
    }
    check_pos
}
pub fn part_two(input: &str) -> Option<u32> {
    let (mut map, commands, mut robot_pos) = parse_input(input, &|c, p| match c {
        '.' => (vec![Tile::Blank, Tile::Blank], None),
        '#' => (vec![Tile::Wall, Tile::Wall], None),
        '@' => (vec![Tile::Blank, Tile::Blank], Some((p.0, p.1 * 2))),
        'O' => (vec![Tile::Box(true), Tile::Box(false)], None),
        _ => panic!(),
    });
    for command in commands {
        let delta = match command {
            'v' => (1, 0),
            '<' => (0, -1),
            '>' => (0, 1),
            '^' => (-1, 0),
            _ => panic!(),
        };
        if can_push(&map, robot_pos, delta) {
            robot_pos = move_delta(&mut map, robot_pos, delta);
        }
    }
    Some(
        map.iter()
            .enumerate()
            .map(|(y, r)| {
                r.iter().enumerate().map(move |(x, c)| {
                    if let Tile::Box(left) = c {
                        if *left {
                            100 * y + x
                        } else {
                            0
                        }
                    } else {
                        0
                    }
                })
            })
            .flatten()
            .sum::<usize>() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
