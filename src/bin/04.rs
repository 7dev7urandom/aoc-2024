advent_of_code::solution!(4);
pub fn part_one(input: &str) -> Option<u32> {
    let word_search_grid: Vec<Vec<char>> =
        input.lines().map(|line| line.chars().collect()).collect();
    let mut total = 0;
    let search_str: Vec<char> = vec!['M', 'A', 'S'];
    const XMAS_PATHS: [(i32, i32); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    for x in 0..word_search_grid.len() {
        for y in 0..word_search_grid[0].len() {
            let mut total_found = 0;
            if word_search_grid[x][y] != 'X' {
                continue;
            }
            for (dx, dy) in XMAS_PATHS.iter() {
                let mut x = x as i32;
                let mut y = y as i32;
                let mut found = true;
                for letter in search_str.iter() {
                    x += dx;
                    y += dy;
                    if x < 0
                        || y < 0
                        || x >= word_search_grid.len() as i32
                        || y >= word_search_grid[0].len() as i32
                    {
                        found = false;
                        break;
                    }
                    if word_search_grid[x as usize][y as usize] != *letter {
                        found = false;
                        break;
                    }
                }
                if found {
                    total_found += 1;
                }
            }
            // print!("{}", total_found);
            total += total_found;
        }
        // println!();
    }
    Some(total as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    const MAS_PATHS: [(i32, i32); 2] = [(-1, -1), (-1, 1)];
    let word_search_grid: Vec<Vec<char>> =
        input.lines().map(|line| line.chars().collect()).collect();
    let mut total = 0;
    for x in 1..(word_search_grid.len() - 1) {
        for y in 1..(word_search_grid[0].len() - 1) {
            let mut found = true;
            if word_search_grid[x][y] != 'A' {
                found = false;
            }
            for (dx, dy) in MAS_PATHS.iter() {
                if word_search_grid[(x as i32 + dx) as usize][(y as i32 + dy) as usize] != 'M'
                    || word_search_grid[(x as i32 - dx) as usize][(y as i32 - dy) as usize] != 'S'
                {
                    if word_search_grid[(x as i32 + dx) as usize][(y as i32 + dy) as usize] != 'S'
                        || word_search_grid[(x as i32 - dx) as usize][(y as i32 - dy) as usize]
                            != 'M'
                    {
                        found = false;
                        break;
                    }
                }
            }
            // print!("{}", if found { 1 } else { 0 });
            if found {
                total += 1;
            }
        }
        // println!();
    }
    Some(total as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
