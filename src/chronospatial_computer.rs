use itertools::Itertools;
use pathfinding::directed::dfs::dfs;
use std::string::ToString;

// ------------------------------------------------------------------------------------------------
// Exports

pub fn program_output(input: &str) -> String {
    let (mut registers, program) = computer_registers_and_program(input);

    // Just execute program and join output
    interpret_program(&mut registers, &program)
        .iter()
        .map(ToString::to_string)
        .join(",")
}

pub fn program_quine_register_value(input: &str) -> usize {
    let (registers, program) = computer_registers_and_program(input);

    // DFS through possible octal digits adding shifted register A nodes
    // only when output partially matches original program
    dfs(
        (0, program.len() - 1),
        |&(a, program_skip)| {
            let mut successors = vec![];
            let mut registers = registers.clone();
            registers[0] = a;

            if interpret_program(&mut registers, &program)[0] == program[program_skip] {
                successors.push((if program_skip == 0 { a } else { a << 3 }, program_skip - 1));
            }

            if a & 7 < 7 {
                successors.push((a & !7 | ((a & 7) + 1) & 7, program_skip));
            }

            successors
        },
        |&(_, program_skip)| program_skip == usize::MAX,
    )
    .expect("Expected quine solution")
    .last()
    .expect("Expected quine node")
    .0
}

// ------------------------------------------------------------------------------------------------
// Functions

fn interpret_program(registers: &mut [usize], program: &[usize]) -> Vec<usize> {
    // Initialize program counter and output vector
    let (mut program_counter, mut output) = (0, vec![]);

    // Interpret instructions
    while let Some(&instruction) = program.get(program_counter) {
        // Get literal and combo operators
        let literal = program[program_counter + 1];
        let combo = match literal {
            0..=3 => literal,
            4..=6 => registers[literal - 4],
            _ => usize::MAX,
        };

        match instruction {
            // adv
            0 => registers[0] >>= combo,
            // bxl
            1 => registers[1] ^= literal,
            // bst
            2 => registers[1] = combo & 7,
            // jnz
            3 => {
                if registers[0] != 0 {
                    program_counter = literal;
                    continue;
                }
            }
            // bxc
            4 => registers[1] ^= registers[2],
            // out
            5 => output.push(combo & 7),
            // bdv
            6 => registers[1] = registers[0] >> combo,
            // cdv
            7 => registers[2] = registers[0] >> combo,
            _ => unreachable!("Invalid program instruction"),
        }

        program_counter += 2;
    }

    output
}

// ------------------------------------------------------------------------------------------------
// Parsers

fn computer_registers_and_program(input: &str) -> (Vec<usize>, Vec<usize>) {
    let (registers, program) = input.split_once("\n\n").expect("Expected two sections");

    // Parse registers and program instructions
    (
        registers
            .lines()
            .map(|line| {
                line.split_once(": ")
                    .expect("Expected registers init")
                    .1
                    .parse::<usize>()
                    .expect("Expected valid integer")
            })
            .collect_vec(),
        program
            .split_once(": ")
            .expect("Expected program instructions")
            .1
            .replace('\n', "")
            .split(',')
            .map(|n| n.parse::<usize>().expect("Expected valid integer"))
            .collect_vec(),
    )
}
