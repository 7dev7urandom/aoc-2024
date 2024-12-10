advent_of_code::solution!(9);
enum BlockElement {
    // id
    Data(u32),
    Empty,
}
#[derive(Debug)]
enum FileElement {
    // index, size, id
    Data(u32, u32, u32),
    // index, size
    Empty(u32, u32),
}
fn parse_input_blocks(input: &str) -> Vec<BlockElement> {
    let mut chrs = vec![];
    let mut index = 0;
    input.chars().enumerate().for_each(|(i, c)| {
        let number_of_entries = c.to_digit(10).unwrap();
        if number_of_entries == 0 {
            return;
        }
        if i % 2 == 0 {
            let block_id = i / 2;
            for _ in 0..number_of_entries {
                chrs.push(BlockElement::Data(block_id as u32));
            }
            index += number_of_entries;
        } else {
            for _ in 0..number_of_entries {
                chrs.push(BlockElement::Empty);
            }
            index += number_of_entries;
        }
    });
    chrs
}
fn parse_input_files(input: &str) -> Vec<FileElement> {
    let mut chrs = vec![];
    let mut index = 0;
    input.chars().enumerate().for_each(|(i, c)| {
        let number_of_entries = c.to_digit(10).unwrap();
        if number_of_entries == 0 {
            return;
        }
        if i % 2 == 0 {
            let block_id = i / 2;
            chrs.push(FileElement::Data(index, number_of_entries, block_id as u32));
            index += number_of_entries;
        } else {
            chrs.push(FileElement::Empty(index, number_of_entries));
            index += number_of_entries;
        }
    });
    chrs
}
pub fn part_one(input: &str) -> Option<u64> {
    let mut data = parse_input_blocks(input);
    let mut empty_pointer = 0;
    let mut data_pointer = data.len() - 1;
    // Rather efficient. Iterates over the entire data array once, and swaps the data and empty
    // O(n) time complexity
    loop {
        if data_pointer <= empty_pointer {
            break;
        }
        if let BlockElement::Data(_) = data[empty_pointer] {
            empty_pointer += 1;
            continue;
        }
        if let BlockElement::Empty = data[data_pointer] {
            data_pointer -= 1;
            continue;
        }
        if let BlockElement::Data(chr) = data[data_pointer] {
            data[data_pointer] = BlockElement::Empty;
            data[empty_pointer] = BlockElement::Data(chr);
        }
    }
    let checksum = data.iter().enumerate().fold(0u64, |acc, (i, c)| {
        if let BlockElement::Empty = c {
            acc
        } else if let BlockElement::Data(chr) = c {
            acc + (i as u64 * *chr as u64)
        } else {
            acc
        }
    });
    Some(checksum)
}
#[allow(dead_code)]
fn print_file_element_str(data: &Vec<FileElement>) {
    for i in data.iter() {
        match i {
            FileElement::Data(.., size, id) => {
                print!("{}", id.to_string().repeat(*size as usize));
            }
            FileElement::Empty(_, size, ..) => {
                print!("{}", ".".to_string().repeat(*size as usize));
            }
        }
    }
    println!();
}
pub fn part_two(input: &str) -> Option<u64> {
    let mut data = parse_input_files(input);
    for i in (0..data.len()).rev() {
        'a: loop {
            if let FileElement::Data(d_index, d_size, id) = data[i] {
                for j in 0..i {
                    if let FileElement::Empty(e_index, e_size) = data[j] {
                        if e_size == d_size {
                            data[j] = FileElement::Data(e_index, d_size, id);
                            data[i] = FileElement::Empty(d_index, d_size);
                            break 'a;
                        } else if e_size > d_size {
                            data[j] = FileElement::Data(e_index, d_size, id);
                            data.insert(
                                j + 1,
                                FileElement::Empty(e_index + d_size, e_size - d_size),
                            );
                            data[i + 1] = FileElement::Empty(d_index, d_size);
                            continue 'a;
                        }
                    }
                }
            }
            break;
        }
    }

    let checksum = data.iter().enumerate().fold(0u64, |acc, (_, c)| {
        if let FileElement::Empty(..) = c {
            acc
        } else if let FileElement::Data(index, size, id) = c {
            // A little arithmetic sequence sum formula
            let index = *index as u64;
            let size = *size as u64;
            acc + (index + index + size - 1) * size / 2 * *id as u64
        } else {
            acc
        }
    });
    Some(checksum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
