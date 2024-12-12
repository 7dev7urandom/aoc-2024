use std::collections::HashMap;

advent_of_code::solution!(12);

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|num| num as u32).collect())
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_input(input);
    let mut region_id_inc = 0;
    let mut region_area_perimeter: HashMap<i32, (i32, i32)> = std::collections::HashMap::new();
    let mut region_chars_to_id = std::collections::HashMap::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            let region_id = if x > 0 && grid[y][x - 1] == cell {
                if y > 0 && grid[y - 1][x] == cell {
                    let left_id = *region_chars_to_id.get(&(x - 1, y)).unwrap();
                    let top_id = *region_chars_to_id.get(&(x, y - 1)).unwrap();
                    if left_id != top_id {
                        let (left_area, left_perimeter) =
                            region_area_perimeter.remove(&left_id).unwrap();
                        let (top_area, top_perimeter) =
                            region_area_perimeter.remove(&top_id).unwrap();
                        let new_area = left_area + top_area;
                        let new_perimeter = left_perimeter + top_perimeter;
                        region_area_perimeter.insert(left_id, (new_area, new_perimeter));
                        for (_, v) in region_chars_to_id.iter_mut() {
                            if *v == top_id {
                                *v = left_id;
                            }
                        }
                    }
                    left_id
                } else {
                    *region_chars_to_id.get(&(x - 1, y)).unwrap()
                }
            } else if y > 0 && grid[y - 1][x] == cell {
                *region_chars_to_id.get(&(x, y - 1)).unwrap()
            } else {
                region_id_inc += 1;
                region_chars_to_id.insert((x, y), region_id_inc);
                region_id_inc
            };
            region_chars_to_id.insert((x, y), region_id);
            let mut perimeter_to_add = 0;
            if x == 0 || grid[y][x - 1] != cell {
                perimeter_to_add += 1;
            }
            if x == grid[y].len() - 1 || grid[y][x + 1] != cell {
                perimeter_to_add += 1;
            }
            if y == 0 || grid[y - 1][x] != cell {
                perimeter_to_add += 1;
            }
            if y == grid.len() - 1 || grid[y + 1][x] != cell {
                perimeter_to_add += 1;
            }
            let area_to_add = 1;
            region_area_perimeter
                .entry(region_id)
                .and_modify(|e| {
                    e.0 += area_to_add;
                    e.1 += perimeter_to_add;
                })
                .or_insert((area_to_add, perimeter_to_add));
        }
    }
    Some(
        region_area_perimeter
            .iter()
            .map(|(_, (area, perimeter))| area * perimeter)
            .sum::<i32>() as u32,
    )
}
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_input(input);
    let mut region_id_inc = 0;
    let mut region_area_edges: HashMap<i32, (i32, [Vec<(i32, i32)>; 4])> =
        std::collections::HashMap::new();
    let mut region_chars_to_id = std::collections::HashMap::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            let region_id = if x > 0 && grid[y][x - 1] == cell {
                if y > 0 && grid[y - 1][x] == cell {
                    let left_id = *region_chars_to_id.get(&(x - 1, y)).unwrap();
                    let top_id = *region_chars_to_id.get(&(x, y - 1)).unwrap();
                    if left_id != top_id {
                        let (left_area, mut left_perimeter) =
                            region_area_edges.remove(&left_id).unwrap();
                        let (top_area, top_perimeter) = region_area_edges.remove(&top_id).unwrap();
                        let new_area = left_area + top_area;
                        for (i, edge) in left_perimeter.iter_mut().enumerate() {
                            edge.extend(top_perimeter[i].iter());
                        }
                        region_area_edges.insert(left_id, (new_area, left_perimeter));
                        for (_, v) in region_chars_to_id.iter_mut() {
                            if *v == top_id {
                                *v = left_id;
                            }
                        }
                    }
                    left_id
                } else {
                    *region_chars_to_id.get(&(x - 1, y)).unwrap()
                }
            } else if y > 0 && grid[y - 1][x] == cell {
                *region_chars_to_id.get(&(x, y - 1)).unwrap()
            } else {
                region_id_inc += 1;
                region_chars_to_id.insert((x, y), region_id_inc);
                region_id_inc
            };
            region_chars_to_id.insert((x, y), region_id);
            let mut edges_to_add = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];
            if x == 0 || grid[y][x - 1] != cell {
                edges_to_add[Direction::Left as usize].push((x as i32, y as i32));
            }
            if x == grid[y].len() - 1 || grid[y][x + 1] != cell {
                edges_to_add[Direction::Right as usize].push((x as i32, y as i32));
            }
            if y == 0 || grid[y - 1][x] != cell {
                edges_to_add[Direction::Up as usize].push((x as i32, y as i32));
            }
            if y == grid.len() - 1 || grid[y + 1][x] != cell {
                edges_to_add[Direction::Down as usize].push((x as i32, y as i32));
            }
            let area_to_add = 1;
            region_area_edges
                .entry(region_id)
                .and_modify(|e| {
                    e.0 += area_to_add;
                    e.1.iter_mut()
                        .zip(edges_to_add.iter())
                        .for_each(|(e, edges)| {
                            e.extend(edges);
                        });
                })
                .or_insert((area_to_add, edges_to_add));
        }
    }
    for region in region_area_edges.iter_mut() {
        let (_, (_, edges)) = region;
        for (i, edge) in edges.iter_mut().enumerate() {
            let copy = edge.clone();
            for x in 0..copy.len() {
                for y in (x..copy.len()).rev() {
                    if i == Direction::Up as usize || i == Direction::Down as usize {
                        if copy[x].0.abs_diff(copy[y].0) == 1 && copy[x].1 == copy[y].1 {
                            edge.iter()
                                .position(|e| *e == copy[y])
                                .map(|i| edge.remove(i));
                        }
                    } else {
                        if copy[x].1.abs_diff(copy[y].1) == 1 && copy[x].0 == copy[y].0 {
                            edge.iter()
                                .position(|e| *e == copy[y])
                                .map(|i| edge.remove(i));
                        }
                    }
                }
            }
        }
    }
    Some(
        region_area_edges
            .iter()
            .map(|(_, (area, edges))| area * edges.iter().map(|e| e.len() as i32).sum::<i32>())
            .sum::<i32>() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
