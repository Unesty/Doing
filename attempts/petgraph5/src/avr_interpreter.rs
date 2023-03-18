
pub struct ProcessorState {
    memory1: [u16; 65536], // 16-bit memory with 65536 addresses
    memory2: [u16; 65536], // 16-bit memory with 65536 addresses
    exestart: u16,
    exeend: u16,
    pc: u16, // program counter
}
fn mov(state: &mut ProcessorState, dest: u16, src: u16) {
    state.memory2[dest as usize] = state.memory1[src as usize];
    state.pc += 3;
}
fn cmp(state: &mut ProcessorState, dest: u16, src: u16) {
    // Compare values at addresses a and b
    // Set some flag in the processor state based on the comparison result
    state.memory2[dest as usize] = (state.memory1[dest as usize] < state.memory1[src as usize]) as u16;
    state.pc += 3;
}

fn not(state: &mut ProcessorState, dest: u16) {
    state.memory2[dest as usize] = !state.memory1[dest as usize];
    state.pc += 3;
}

fn and(state: &mut ProcessorState, dest: u16, src: u16) {
    state.memory2[dest as usize] &= state.memory1[src as usize];
    state.pc += 3;
}

fn or(state: &mut ProcessorState, dest: u16, src: u16) {
    state.memory2[dest as usize] |= state.memory1[src as usize];
    state.pc += 3;
}

fn nand(state: &mut ProcessorState, dest: u16, src: u16) {
    state.memory2[dest as usize] &= !state.memory1[src as usize];
    state.pc += 3;
}

fn xor(state: &mut ProcessorState, dest: u16, src: u16) {
    state.memory2[dest as usize] ^= state.memory1[src as usize];
    state.pc += 3;
}

fn add(state: &mut ProcessorState, dest: u16, src: u16) {
    state.memory2[dest as usize] = state.memory1[dest as usize].wrapping_add(state.memory1[src as usize]);
    state.pc += 3;
}

fn sub(state: &mut ProcessorState, dest: u16, src: u16) {
    state.memory2[dest as usize] = state.memory1[dest as usize].wrapping_sub(state.memory1[src as usize]);
    state.pc += 3;
}

fn mul(state: &mut ProcessorState, dest: u16, src: u16) {
    state.memory2[dest as usize] = state.memory1[dest as usize].wrapping_mul(state.memory1[src as usize]);
    state.pc += 3;
}

fn div(state: &mut ProcessorState, dest: u16, src: u16) {
    if state.memory1[src as usize] == 0 {
        state.pc += 3;
        // println!("div by 0");
    } else {
        state.memory2[dest as usize] = state.memory1[dest as usize].wrapping_div(state.memory1[src as usize]);
        state.pc += 3;
    }
}
