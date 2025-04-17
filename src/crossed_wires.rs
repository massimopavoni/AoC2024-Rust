use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

use crate::random_utils::FxHashWithCapacity;

// ------------------------------------------------------------------------------------------------
// Exports

pub fn final_z_wires_value(input: &str) -> u64 {
    // Recursive output calculation
    fn calculate_output(output: [u8; 3], inputs: &mut Inputs, gates: &Gates) -> bool {
        if let Some(&value) = inputs.get(&output) {
            return value;
        }

        let &(operation, input_a, input_b) = gates.get(&output).expect("Gate output not found");

        let a_value = calculate_output(input_a, inputs, gates);
        let b_value = calculate_output(input_b, inputs, gates);

        let output_value = match operation {
            0 => a_value && b_value,
            1 => a_value || b_value,
            2 => a_value ^ b_value,
            _ => unreachable!("Unknown gate operation"),
        };

        inputs.insert(output, output_value);

        output_value
    }

    let (mut inputs, gates) = circuit_inputs_and_gates(input);

    let mut z_output = 0;

    // Retrieve z wires value
    for (bit, &output) in gates
        .keys()
        .filter(|output| output[0] == b'z')
        .sorted_unstable()
        .enumerate()
    {
        if calculate_output(output, &mut inputs, &gates) {
            z_output += 1 << bit;
        }
    }

    z_output
}

pub fn ripple_carry_adder_swapped_wires(input: &str) -> String {
    let (_, gates) = circuit_inputs_and_gates(input);

    let (mut or_inputs, mut xor_inputs) =
        (FxHashSet::with_capacity(96), FxHashSet::with_capacity(192));

    let mut z_msb = b"z00";

    // Prepare or/xor inputs and find z wires most significant bit
    for (output, (operation, input_a, input_b)) in &gates {
        match operation {
            0 => {}
            1 => {
                or_inputs.insert(input_a);
                or_inputs.insert(input_b);
            }
            2 => {
                xor_inputs.insert(input_a);
                xor_inputs.insert(input_b);
            }
            _ => unreachable!("Unknown gate operation"),
        }

        if output >= z_msb {
            z_msb = output;
        }
    }

    let mut swapped_wires = Vec::with_capacity(8);

    // Find swapped wires by enforcing puzzle input constraints
    for (output, (operation, input_a, input_b)) in &gates {
        match operation {
            0 => {
                // AND cannot output z wires
                // AND outputs must be OR inputs, unless half adder
                if output[0] == b'z'
                    || input_a != b"x00" && input_a != b"y00" && !or_inputs.contains(&output)
                {
                    swapped_wires.push(output);
                }
            }
            1 => {
                // OR cannot output z wires, unless last full adder
                if output[0] == b'z' && output != z_msb {
                    swapped_wires.push(output);
                }
            }
            2 => match (input_a[0], input_b[0], output[0]) {
                (b'x', b'y', _) | (b'y', b'x', _) => {
                    // XOR output must be second XOR input, unless half adder
                    if output != b"z00" && !xor_inputs.contains(&output) {
                        swapped_wires.push(output);
                    }
                }
                // XOR output can be z if second gate
                (_, _, b'z') => {}
                // All other cases are wrong
                _ => swapped_wires.push(output),
            },
            _ => unreachable!("Unknown gate operation"),
        }
    }

    // Retrieve swapped wires names
    swapped_wires.sort_unstable();
    Itertools::intersperse(
        swapped_wires
            .into_iter()
            .map(|wire| String::from_utf8_lossy(wire).into_owned()),
        ",".to_string(),
    )
    .collect()
}

// ------------------------------------------------------------------------------------------------
// Parsers

type Inputs = FxHashMap<[u8; 3], bool>;
type Gates = FxHashMap<[u8; 3], (u8, [u8; 3], [u8; 3])>;

fn circuit_inputs_and_gates(input: &str) -> (Inputs, Gates) {
    let (inputs, gates) = input.split_once("\n\n").expect("Expected two sections");

    // Parse inputs and circuit gates
    (
        inputs
            .lines()
            .map(|line| {
                (
                    line.as_bytes()[0..3].try_into().expect("Expected 3 bytes"),
                    line.as_bytes()[5] == b'1',
                )
            })
            .collect(),
        gates
            .lines()
            .map(|line| {
                let parts = line.as_bytes().split(|&b| b == b' ').collect_vec();

                (
                    parts[4].try_into().expect("Expected 3 bytes"),
                    (
                        match parts.get(1) {
                            Some(&b"AND") => 0,
                            Some(&b"OR") => 1,
                            Some(&b"XOR") => 2,
                            _ => unreachable!("Unknown gate operation"),
                        },
                        parts[0].try_into().expect("Expected 3 bytes"),
                        parts[2].try_into().expect("Expected 3 bytes"),
                    ),
                )
            })
            .collect(),
    )
}
