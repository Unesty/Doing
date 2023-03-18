use crate::avr_interpreter::{ProcessorState, execute_instruction};

// Generate all valid states from a list of instructions
pub fn generate_states(instructions: &[u8]) -> Vec<ProcessorState> {
    let mut states = vec![];
    for i in 0..256 {
        let mut state = ProcessorState::new();
        state.pc = 0;
        state.registers[0] = i;
        for &instr in instructions {
            execute_instruction(&mut state, instr);
        }
        states.push(state);
    }
    states
}

// Count the number of possibilities for a processor state
pub fn count_possibilities(state: &ProcessorState) -> u32 {
    let mut count = 0;
    let mut visited = vec![false; 256];
    let mut current_state = state.clone();
    visited[current_state.registers[0] as usize] = true;
    while !visited[current_state.registers[0] as usize] {
        visited[current_state.registers[0] as usize] = true;
        current_state.pc = 0;
        for &instr in &current_state.memory {
            execute_instruction(&mut current_state, instr);
        }
        count += 1;
    }
    count
}

// Calculates the number of possibilities of the system starting from the given state.
pub fn count_possibilities2(state: &ProcessorState) -> u64 {
    // Create a memory region with all instructions set to zero
    let mut mem_region = MemoryRegion::default();

    // Set the instructions in the memory region to match the state
    for (i, &instr) in state.instructions.iter().enumerate() {
        mem_region.set_instruction(i as u8, instr);
    }

    // Execute the memory region and count the number of times each instruction is executed
    let mut instruction_counts = [0; 256];
    let mut pc = 0;

    while pc < mem_region.len() {
        let instr = mem_region.get_instruction(pc).unwrap();
        instruction_counts[instr as usize] += 1;
        pc += 1;
    }

    // Calculate the number of possibilities as the product of the number of times each instruction is executed
    let mut possibilities = 1;

    for count in instruction_counts.iter() {
        if *count > 0 {
            possibilities *= *count;
        }
    }

    possibilities
}
// Compare two processor states and return a score
// A higher score indicates a state with better survival chances
pub fn compare_states(state1: &ProcessorState, state2: &ProcessorState) -> u32 {
    let count1 = count_possibilities(state1);
    let count2 = count_possibilities(state2);
    if count1 == count2 {
        return 0;
    }
    if count1 > count2 {
        count1 - count2
    } else {
        count2 - count1
    }
}
