use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

advent_of_code::solution!(24);

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum GateType {
    And,
    Or,
    Xor,
}
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Gate {
    gate_type: GateType,
    input1: String,
    input2: String,
    output: String,
}
impl Gate {
    fn process(
        &self,
        wires: &mut HashMap<String, bool>,
        wire_to_gate: &HashMap<String, Vec<Rc<Gate>>>,
    ) {
        if !wires.contains_key(&self.input1) || !wires.contains_key(&self.input2) {
            return;
        }
        let input1 = wires.get(&self.input1).unwrap();
        let input2 = wires.get(&self.input2).unwrap();
        let result = match self.gate_type {
            GateType::And => input1 & input2,
            GateType::Or => input1 | input2,
            GateType::Xor => input1 ^ input2,
        };
        wires.insert(self.output.clone(), result);
        if let Some(gates) = wire_to_gate.get(&self.output) {
            gates
                .iter()
                .for_each(|gate| gate.process(wires, wire_to_gate));
        }
    }
}
fn parse_input(input: &str) -> (HashMap<String, bool>, HashMap<String, Vec<Rc<Gate>>>, u32) {
    let mut max_z = 0;
    let mut wires: HashMap<String, bool> = HashMap::new();
    let mut wire_to_gate: HashMap<String, Vec<Rc<Gate>>> = HashMap::new();
    let (p1, p2) = input.split_once("\n\n").unwrap();
    p1.lines().for_each(|l| {
        let (wire_name, value) = l.split_once(": ").unwrap();
        wires.insert(wire_name.to_string(), value == "1");
    });
    for gate_str in p2.lines() {
        let (inputs, output) = gate_str.split_once(" -> ").unwrap();
        let (input1, operation, input2) = inputs.split(" ").collect_tuple().unwrap();
        let gate_type = match operation {
            "AND" => GateType::And,
            "OR" => GateType::Or,
            "XOR" => GateType::Xor,
            _ => panic!("Invalid gate type"),
        };
        let gate = Rc::new(Gate {
            gate_type,
            input1: input1.to_string(),
            input2: input2.to_string(),
            output: output.to_string(),
        });
        if output.chars().next().unwrap() == 'z' {
            max_z = max_z.max(output[1..].parse().unwrap());
        }
        wire_to_gate
            .entry(input1.to_string())
            .or_insert_with(Vec::new)
            .push(Rc::clone(&gate));
        wire_to_gate
            .entry(input2.to_string())
            .or_insert_with(Vec::new)
            .push(Rc::clone(&gate));
    }
    (wires, wire_to_gate, max_z)
}
fn simulate(
    orig_wires: &HashMap<String, bool>,
    wire_to_gate: &HashMap<String, Vec<Rc<Gate>>>,
) -> HashMap<String, bool> {
    let mut wires = HashMap::new();
    for wire in orig_wires {
        wires.insert(wire.0.clone(), *wire.1);
        wire_to_gate
            .get(wire.0)
            .unwrap_or(&vec![])
            .iter()
            .for_each(|gate| gate.process(&mut wires, &wire_to_gate));
    }
    wires
}
pub fn part_one(input: &str) -> Option<u64> {
    let (orig_wires, wire_to_gate, max_z) = parse_input(input);
    let wires = simulate(&orig_wires, &wire_to_gate);
    let mut acc: u64 = 0;
    for i in 0..=max_z {
        if let Some(value) = wires.get(&format!("z{:0>2}", i)) {
            acc = acc | (if *value { 1 } else { 0 } << i);
        }
    }
    Some(acc)
}

fn find_first_incorrect_bit(
    start_z: u32,
    max_z: u32,
    wire_to_gate: &HashMap<String, Vec<Rc<Gate>>>,
) -> Option<u32> {
    let mut wires = HashMap::new();
    for i in (start_z.max(1) - 1)..(max_z - 1) {
        let parts = vec![0b00, 0b01, 0b10, 0b11];
        let a_set: Vec<u64> = parts.iter().map(|p| p << i).collect();
        let b_set: Vec<u64> = parts.iter().map(|p| p << i).collect();
        for a in a_set {
            for b in b_set.iter() {
                for w in 0..=max_z {
                    wires.insert(format!("x{:0>2}", w), (a >> w) & 1 == 1);
                    wires.insert(format!("y{:0>2}", w), (b >> w) & 1 == 1);
                }
                let out_wires = simulate(&wires, wire_to_gate);
                let total = a + b;
                if let Some(returned) = out_wires.get(&format!("z{:0>2}", i + 1)) {
                    if *returned != ((total >> (i + 1)) & 1 == 1) {
                        return Some(i + 1);
                    }
                }
            }
        }
    }
    None
}
pub fn part_two(input: &str) -> Option<String> {
    let (_, wire_to_gate, max_z) = parse_input(input);
    let mut all_gates = wire_to_gate
        .values()
        .flatten()
        .cloned()
        .collect::<HashSet<_>>();
    // Should be an addition machine
    // works like this:

    // x00 XOR y00 -> z00 (== s00)
    // x00 AND y00 -> w00

    // x01 XOR y01 -> s01
    // x01 AND y01 -> c01
    // w00 XOR s01 -> z01
    // w00 AND s01 -> o01
    // c01 OR o01 -> w01

    // x02 XOR y02 -> s02
    // x02 AND y02 -> c02
    // w01 XOR s02 -> z02
    // w01 AND s02 -> o02
    // c02 OR o02 -> w02

    let mut swapped_gates = vec![];
    let mut next_incor_bit = find_first_incorrect_bit(0, max_z, &wire_to_gate);
    while let Some(next_incorrect) = next_incor_bit {
        let mut revelant_gates = vec![];

        // relevant gates
        // x{incorrect_bit} XOR y{incorrect_bit} -> aaa
        // x{incorrect_bit} AND y{incorrect_bit} -> bbb
        // ccc XOR aaa -> z{incorrect_bit}
        // ccc AND aaa -> ddd
        // bbb OR ddd -> ccn
        let x = format!("x{:0>2}", next_incorrect);
        let y = format!("y{:0>2}", next_incorrect);
        revelant_gates.push(
            all_gates
                .iter()
                .find(|gate| {
                    ((gate.input1 == x && gate.input2 == y)
                        || (gate.input1 == y && gate.input2 == x))
                        && gate.gate_type == GateType::Xor
                })
                .unwrap()
                .clone(),
        );
        let a = revelant_gates[0].output.clone();
        revelant_gates.push(
            all_gates
                .iter()
                .find(|gate| {
                    ((gate.input1 == x && gate.input2 == y)
                        || (gate.input1 == y && gate.input2 == x))
                        && gate.gate_type == GateType::And
                })
                .unwrap()
                .clone(),
        );
        let b = revelant_gates[1].output.clone();
        if let Some(x) = all_gates.iter().find(|gate| {
            gate.output == format!("z{:0>2}", next_incorrect) && gate.gate_type == GateType::Xor
        }) {
            revelant_gates.push(x.clone());
        }
        if let Some(x) = all_gates
            .iter()
            .find(|gate| (gate.input1 == a || gate.input2 == a) && gate.gate_type == GateType::Xor)
        {
            revelant_gates.push(x.clone());
        }
        if let Some(x) = all_gates
            .iter()
            .find(|gate| (gate.input1 == a || gate.input2 == a) && gate.gate_type == GateType::And)
        {
            revelant_gates.push(x.clone());
        }
        if let Some(x) = all_gates
            .iter()
            .find(|gate| (gate.input1 == b || gate.input2 == b) && gate.gate_type == GateType::Or)
        {
            revelant_gates.push(x.clone());
        }
        // try swapping all pairs of relevant gate outputs
        'swaps: for i in 0..revelant_gates.len() {
            for j in i + 1..revelant_gates.len() {
                let i_output = revelant_gates[i].output.clone();
                let j_output = revelant_gates[j].output.clone();
                // Recreate the wire_to_gate map, deep cloning data inside the Rc
                let mut new_wire_to_gate = HashMap::new();
                for gate in all_gates.iter() {
                    let new_gate = Rc::new(Gate {
                        gate_type: gate.gate_type.clone(),
                        input1: gate.input1.clone(),
                        input2: gate.input2.clone(),
                        output: if gate.output == i_output {
                            j_output.clone()
                        } else if gate.output == j_output {
                            i_output.clone()
                        } else {
                            gate.output.clone()
                        },
                    });
                    new_wire_to_gate
                        .entry(gate.input1.clone())
                        .or_insert_with(Vec::new)
                        .push(Rc::clone(&new_gate));
                    new_wire_to_gate
                        .entry(gate.input2.clone())
                        .or_insert_with(Vec::new)
                        .push(Rc::clone(&new_gate));
                }
                // Check if the incorrect bits are still incorrect
                let new_next_incorrect =
                    find_first_incorrect_bit(next_incorrect, max_z, &new_wire_to_gate);
                // +1 to avoid the false positive where this bit is now correct but the carry bit is still wrong
                // may cause issues if consecutive bits contain swaps
                if new_next_incorrect.is_none_or(|x| x > next_incorrect + 1) {
                    swapped_gates.push(i_output);
                    swapped_gates.push(j_output);
                    next_incor_bit = new_next_incorrect;
                    all_gates = new_wire_to_gate
                        .values()
                        .flatten()
                        .cloned()
                        .collect::<HashSet<_>>();
                    break 'swaps;
                }
            }
        }
    }
    Some(swapped_gates.iter().sorted().join(","))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(2024));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some("c01,o03,s01,z03".to_string()));
    }
}
