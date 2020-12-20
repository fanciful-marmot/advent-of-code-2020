use std::env;
use std::fs;

#[derive(PartialEq, Eq, Copy, Clone)]
enum Operation {
    ACC,
    JMP,
    NOP,
}

struct Instruction {
    pub operation: Operation,
    pub value: i16,
    pub marked: bool,
}

// (acc, true) if successful, (acc, false) if looped
fn run(program: &mut Vec<Instruction>) -> (i32, bool) {
    let mut acc: i32 = 0;
    let mut looped = false;

    // Mark all instructions as clean
    for i in 0..program.len() {
        program[i].marked = false;
    }

    // Run
    let mut instruction_pointer: i32 = 0;
    while instruction_pointer < (program.len() as i32) {
        let instruction = &mut program[instruction_pointer as usize];

        if instruction.marked {
            looped = true;
            break;
        }

        instruction.marked = true;

        match instruction.operation {
            Operation::ACC => {
                acc += instruction.value as i32;
                instruction_pointer += 1;
            }
            Operation::JMP => {
                instruction_pointer += instruction.value as i32;
            }
            Operation::NOP => {
                instruction_pointer += 1;
            }
        }
    }

    (acc, looped)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("Reading input file {}", filename);

    let contents = fs::read_to_string(filename).expect("Failed to read file");

    let mut program: Vec<Instruction> = contents
        .lines()
        .map(|line| {
            let segments: Vec<&str> = line.split(' ').collect();
            let operation = match segments[0] {
                "acc" => Operation::ACC,
                "jmp" => Operation::JMP,
                _ => Operation::NOP,
            };

            let value = segments[1].parse::<i16>().unwrap();

            Instruction {
                operation,
                value,
                marked: false,
            }
        })
        .collect();

    // Find all indices to potentially change the operation
    let fixable_indices: Vec<usize> = program
        .iter()
        .enumerate()
        .filter(|(_, instr)| instr.operation != Operation::ACC)
        .map(|(i, _)| i)
        .collect();

    for &i in fixable_indices.iter() {
        let instr = &mut program[i];
        let old_op = instr.operation;

        // Swap the operation
        instr.operation = if instr.operation == Operation::NOP {
            Operation::JMP
        } else {
            Operation::NOP
        };

        // Run the program
        let (acc, looped) = run(&mut program);
        if !looped {
            println!("Program terminated with acc {}", acc);
        }

        // Fix operation
        program[i].operation = old_op;
    }
}
