advent_of_code::solution!(17);

#[derive(Debug)]
enum Instruction {
    Adv(u8),
    Bxl(u8),
    Bst(u8),
    Jnz(u8),
    Bxc(),
    Out(u8),
    Bdv(u8),
    Cdv(u8),
}
fn parse_input(input: &str) -> ((u64, u64, u64), Vec<Instruction>) {
    let (regs, instr) = input.split_once("\n\n").unwrap();
    let [reg_a, reg_b, reg_c] = regs
        .lines()
        .map(|line| {
            let (_, val) = line.split_once(": ").unwrap();
            val.parse::<u64>().unwrap()
        })
        .collect::<Vec<u64>>()[0..3]
    else {
        panic!()
    };
    let instr: Vec<u8> = instr
        .split_once(": ")
        .unwrap()
        .1
        .split(',')
        .map(|c| c.parse().unwrap())
        .collect();
    let mut instrr = vec![];
    for i in 0..(instr.len() / 2) {
        instrr.push(match instr[i * 2] {
            0 => Instruction::Adv(instr[i * 2 + 1]),
            1 => Instruction::Bxl(instr[i * 2 + 1]),
            2 => Instruction::Bst(instr[i * 2 + 1]),
            3 => Instruction::Jnz(instr[i * 2 + 1]),
            4 => Instruction::Bxc(),
            5 => Instruction::Out(instr[i * 2 + 1]),
            6 => Instruction::Bdv(instr[i * 2 + 1]),
            7 => Instruction::Cdv(instr[i * 2 + 1]),
            _ => panic!(),
        });
    }
    ((reg_a, reg_b, reg_c), instrr)
}
pub fn part_one(input: &str) -> Option<String> {
    let ((mut reg_a, mut reg_b, mut reg_c), instr) = parse_input(input);
    let mut pc = 0;
    let mut output = vec![];
    loop {
        if pc >= instr.len() {
            break;
        }
        if let Some(c) = do_instruction(&instr[pc], &mut reg_a, &mut reg_b, &mut reg_c, &mut pc) {
            output.push(c);
        }
    }
    Some(
        output
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join(","),
    )
}

fn do_instruction(
    instr: &Instruction,
    reg_a: &mut u64,
    reg_b: &mut u64,
    reg_c: &mut u64,
    pc: &mut usize,
) -> Option<u32> {
    fn get_combo(reg_a: u64, reg_b: u64, reg_c: u64, operand: u8) -> u64 {
        match operand {
            0..4 => operand as u64,
            4 => reg_a,
            5 => reg_b,
            6 => reg_c,
            _ => panic!(),
        }
    }
    let mut out: Option<u32> = None;
    match instr {
        Instruction::Adv(val) => {
            if val == &4 && *reg_a > 10u64 {
                *reg_a = 0;
            } else {
                *reg_a = *reg_a / 2u64.pow(get_combo(*reg_a, *reg_b, *reg_c, *val) as u32)
            }
        }
        Instruction::Bxl(val) => *reg_b = *reg_b ^ *val as u64,
        Instruction::Bst(val) => *reg_b = get_combo(*reg_a, *reg_b, *reg_c, *val) % 8,
        Instruction::Jnz(val) => {
            if *reg_a != 0 {
                *pc = *val as usize / 2;
                return None;
            }
        }
        Instruction::Bxc() => *reg_b = *reg_b ^ *reg_c,
        Instruction::Out(val) => {
            // println!("{}", reg_a);
            out = Some((get_combo(*reg_a, *reg_b, *reg_c, *val) % 8) as u32);
        }
        Instruction::Bdv(val) => {
            if val == &5 && *reg_b > 10u64 {
                *reg_b = 0;
            } else {
                *reg_b = *reg_a / 2u64.pow(get_combo(*reg_a, *reg_b, *reg_c, *val) as u32)
            }
        }
        Instruction::Cdv(val) => {
            if val == &6 && *reg_c > 10u64 {
                *reg_c = 0;
            } else {
                *reg_c = *reg_a / 2u64.pow(get_combo(*reg_a, *reg_b, *reg_c, *val) as u32)
            }
        }
    }
    *pc += 1;
    out
}
fn run_until_output(
    instr: &Vec<Instruction>,
    reg_a: &mut u64,
    reg_b: &mut u64,
    reg_c: &mut u64,
    pc: &mut usize,
) -> Option<u32> {
    loop {
        if *pc >= instr.len() {
            break;
        }
        if let Some(c) = do_instruction(&instr[*pc], reg_a, reg_b, reg_c, pc) {
            return Some(c);
        }
    }
    None
}
pub fn part_two(input: &str) -> Option<u64> {
    let prog_digits: Vec<u32> = input
        .split_once("\n\n")
        .unwrap()
        .1
        .split_once(": ")
        .unwrap()
        .1
        .split(",")
        .map(|c| c.parse().unwrap())
        .collect();
    let instr = parse_input(input).1;
    // Ignore the jump instruction
    let a = 0;
    fn recurse(a: u64, nums: &Vec<u32>, l: usize, instr: &Vec<Instruction>) -> u64 {
        for i in 0..8 {
            let mut reg_a = a * 8 + i;
            if let Some(c) = run_until_output(instr, &mut reg_a, &mut 0, &mut 0, &mut 0) {
                if c == nums[l] {
                    if l == nums.len() - 1 {
                        return a * 8 + i;
                    }
                    let res = recurse(a * 8 + i, nums, l + 1, instr);
                    if res != 0 {
                        return res;
                    }
                }
            }
        }
        0
    }
    Some(recurse(
        a,
        &prog_digits.iter().rev().map(|f| *f).collect(),
        0,
        &instr,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(117440));
    }
}
