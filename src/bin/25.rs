advent_of_code::solution!(25);

fn parse_input(input: &str) -> (Vec<[usize; 5]>, Vec<[usize; 5]>) {
    let pieces = input
        .split("\n\n")
        .map(|p| {
            p.split("\n")
                .map(|l| {
                    l.chars()
                        .map(|c| (c == '#'))
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap()
                })
                .collect::<Vec<[bool; 5]>>()
                .try_into()
                .unwrap()
        })
        .collect::<Vec<[[bool; 5]; 7]>>();
    let mut locks = vec![];
    let mut keys = vec![];
    for piece in pieces {
        if piece[0] == [true, true, true, true, true] {
            let mut lock = [0; 5];
            for x in 0..5 {
                for y in 1..7 {
                    if !piece[y][x] {
                        lock[x] = y - 1;
                        break;
                    }
                }
            }
            locks.push(lock);
        } else {
            let mut key = [0; 5];
            for x in 0..5 {
                for y in (0..6).rev() {
                    if !piece[y][x] {
                        key[x] = 5 - y;
                        break;
                    }
                }
            }
            keys.push(key);
        }
    }
    (locks, keys)
}
pub fn part_one(input: &str) -> Option<u32> {
    let (locks, keys) = parse_input(input);
    let mut sum = 0;
    for key in keys.iter() {
        for lock in locks.iter() {
            if (0..5).all(|i| lock[i] + key[i] < 6) {
                sum += 1;
            }
        }
    }
    Some(sum)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
