advent_of_code::solution!(14);

#[cfg(test)]
const DIMENSIONS: (i32, i32) = (11, 7);
#[cfg(not(test))]
const DIMENSIONS: (i32, i32) = (101, 103);

fn parse_input(input: &str) -> Vec<((i32, i32), (i32, i32))> {
    input
        .lines()
        .map(|l| {
            let (p, v) = l.split_once(" ").unwrap();
            let (px, py) = p.split("=").nth(1).unwrap().split_once(",").unwrap();
            let (vx, vy) = v.split("=").nth(1).unwrap().split_once(",").unwrap();
            (
                (px.parse().unwrap(), py.parse().unwrap()),
                (vx.parse().unwrap(), vy.parse().unwrap()),
            )
        })
        .collect()
}

fn calculate_positions_n(robots: &[((i32, i32), (i32, i32))], n: u32) -> Vec<(i32, i32)> {
    robots
        .iter()
        .map(|robot| {
            let (p, v) = robot;
            let (px, py) = p;
            let (vx, vy) = v;
            let x = (px + (vx * n as i32)).rem_euclid(DIMENSIONS.0) as i32;
            let y = (py + (vy * n as i32)).rem_euclid(DIMENSIONS.1) as i32;
            (x, y)
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let robots = parse_input(input);
    let mut sums = [0; 4];
    let final_positions = calculate_positions_n(&robots, 100);
    for (x, y) in final_positions {
        if x < DIMENSIONS.0 / 2 && y < DIMENSIONS.1 / 2 {
            sums[0] += 1;
        } else if x > (DIMENSIONS.0 / 2) && y < DIMENSIONS.1 / 2 {
            sums[1] += 1;
        } else if x < DIMENSIONS.0 / 2 && y > (DIMENSIONS.1 / 2) {
            sums[2] += 1;
        } else if x > DIMENSIONS.0 / 2 && y > DIMENSIONS.1 / 2 {
            sums[3] += 1;
        }
    }
    Some(sums.iter().product())
}

// ###############################
// #.............................#
// #.............................#
// #.............................#
// #.............................#
// #..............#..............#
// #.............###.............#
// #............#####............#
// #...........#######...........#
// #..........#########..........#
// #............#####............#
// #...........#######...........#
// #..........#########..........#
// #.........###########.........#
// #........#############........#
// #..........#########..........#
// #.........###########.........#
// #........#############........#
// #.......###############.......#
// #......#################......#
// #........#############........#
// #.......###############.......#
// #......#################......#
// #.....###################.....#
// #....#####################....#
// #.............###.............#
// #.............###.............#
// #.............###.............#
// #.............................#
// #.............................#
// #.............................#
// #.............................#
// ###############################
fn detect_tree(robots: &Vec<(i32, i32)>) -> bool {
    let mut tree: [[bool; DIMENSIONS.0 as usize]; DIMENSIONS.1 as usize] =
        [[false; DIMENSIONS.0 as usize]; DIMENSIONS.1 as usize];
    robots
        .iter()
        .for_each(|r| tree[r.1 as usize][r.0 as usize] = true);
    for y in 0..DIMENSIONS.1 as usize {
        'a: for x in 0..DIMENSIONS.0 as usize {
            // Just check a 9x5 area for a triangle and assume that's good enough
            // ....#....
            // ...###...
            // ..#####..
            // .#######.
            // #########
            if tree[y][x] && x > 3 && (x as i32) < DIMENSIONS.0 - 4 && y > 3 {
                for cy in 0..5 {
                    for cx in 0..(5 - cy) {
                        if !tree[y - cy][x - cx] || !tree[y - cy][x + cx] {
                            continue 'a;
                        }
                    }
                }
                return true;
            }
        }
    }
    false
}
pub fn part_two(input: &str) -> Option<u32> {
    let robots = parse_input(input);
    let mut i = 0;
    loop {
        if detect_tree(&calculate_positions_n(&robots, i)) {
            return Some(i);
        }
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        // There is no test for part two :(
        // let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(None::<u32>, None);
    }
}
