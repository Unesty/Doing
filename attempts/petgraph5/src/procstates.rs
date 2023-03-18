use std::fs::OpenOptions;
use std::io::{Read};
use std::fs::File;
// use std::os::unix::io::{AsRawFd, FromRawFd};
// use std::ptr::{null_mut, write_bytes};
use std::slice::from_raw_parts_mut;
use std::thread::sleep;
use std::time::Duration;

use byte_slice_cast::*;

use memmap::MmapOptions;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

use petgraph::{csr::Csr,algo};

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

#[derive(Debug)]
enum Op {
    None,
    Mov,
    Cmp,
    Not,
    And,
    Or,
    Nand,
    Xor,
    Add,
    Sub,
    Mul,
    Div,
}

impl From<u16> for Op {
    fn from(val: u16) -> Self {
        match val {
            0 => Op::None,
            1 => Op::Mov,
            2 => Op::Cmp,
            3 => Op::Not,
            4 => Op::And,
            5 => Op::Or,
            6 => Op::Nand,
            7 => Op::Xor,
            8 => Op::Add,
            9 => Op::Sub,
            10 => Op::Mul,
            11 => Op::Div,
            _ => Op::None,
        }
    }
}

pub fn execute(state: &mut ProcessorState) {
    state.pc = state.exestart;
    let mut alive = false;
    while state.pc < state.exeend {
        // if state.memory1[state.pc as usize] > 11 {
            // println!("pc:{} c:{}", state.pc,state.memory1[state.pc as usize]);
        // }
        let opcode: Op = state.memory1[state.pc as usize].into();
        match opcode {
            Op::Mov => {
                let dest = state.memory1[(state.pc + 1) as usize];
                let src = state.memory1[(state.pc + 2) as usize];
                mov(state, dest, src);
                alive = true;
            }
            Op::Cmp => {
                let dest = state.memory1[(state.pc + 1) as usize];
                let src = state.memory1[(state.pc + 2) as usize];
                cmp(state, dest, src);
                alive = true;
            }
            Op::Not => {
                let dest = state.memory1[(state.pc + 1) as usize];
                not(state, dest);
                alive = true;
            }
            Op::And => {
                let dest = state.memory1[(state.pc + 1) as usize];
                let src = state.memory1[(state.pc + 2) as usize];
                and(state, dest, src);
                alive = true;
            }
            Op::Or => {
                let dest = state.memory1[(state.pc + 1) as usize];
                let src = state.memory1[(state.pc + 2) as usize];
                or(state, dest, src);
                alive = true;
            }
            Op::Nand => {
                let dest = state.memory1[(state.pc + 1) as usize];
                let src = state.memory1[(state.pc + 2) as usize];
                nand(state, dest, src);
                alive = true;
            }
            Op::Xor => {
                let dest = state.memory1[(state.pc + 1) as usize];
                let src = state.memory1[(state.pc + 2) as usize];
                xor(state, dest, src);
                alive = true;
            }
            Op::Add => {
                let dest = state.memory1[(state.pc + 1) as usize];
                let src = state.memory1[(state.pc + 2) as usize];
                add(state, dest, src);
                alive = true;
            }
            Op::Sub => {
                let dest = state.memory1[(state.pc + 1) as usize];
                let src = state.memory1[(state.pc + 2) as usize];
                sub(state, dest, src);
                alive = true;
            }
            Op::Mul => {
                let dest = state.memory1[(state.pc + 1) as usize];
                let src = state.memory1[(state.pc + 2) as usize];
                mul(state, dest, src);
                alive = true;
            }
            Op::Div => {
                let dest = state.memory1[(state.pc + 1) as usize];
                let src = state.memory1[(state.pc + 2) as usize];
                div(state, dest, src);
                alive = true;
            }
            Op::None => {
                // skip
                state.pc+=3;
            }
        }
    }
    if !alive {
        println!("dead");
        // std::process::exit(0x0100);
        // sleep(Duration::from_millis(500));
    }
    state.memory1 = state.memory2;
}

fn main() {
    let mut stgraph =
    let mut round = 0;
    let mut state = ProcessorState {
        memory1: [0; 65536],
        memory2: [0; 65536],
        exestart: 12,
        exeend: 12+3*20000,
        pc: 64,
    };
    // Open the file "/tmp/simpleproc.bin"
    let mut file = File::open("/tmp/simpleproc.bin").expect("Failed to open file");

    // Read the contents of the file into the memory1 field of the ProcessorState struct
    file.read_exact(state.memory1.as_mut_byte_slice()).expect("Failed to read file");

    // Print the first 10 bytes of memory1 to verify that the file was read correctly
    println!("{:?}", &state.memory1[0..10]);
    // Initialize memory with some values
    // state.memory1[64] = 1234;
    // state.memory1[65] = 5678;

    // Initialize memory with random values
    // let mut rng = StdRng::seed_from_u64(42);
    // for i in state.exeend+3..65533 {
    //     state.memory1[i as usize] = rng.gen::<u16>().into();
    // }

    // Open the memory-mapped file
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("/tmp/simpleproc.bin")
        .unwrap();
    file.set_len(65536*2).unwrap();
    let mut mmap = unsafe { MmapOptions::new().map_mut(&file).unwrap() };
    let mmap_ptr = mmap.as_mut_ptr();
    let mmap_slice = unsafe { from_raw_parts_mut(mmap_ptr, 65536*2) };
    println!("starting");
    loop {
        // Execute the next state
        execute(&mut state);

        // Copy memory to mmap
        let memory_slice = state.memory1.as_mut_byte_slice();
        mmap_slice.copy_from_slice(memory_slice);

        if round % 100000 == 0 {
            // Fill executable memory with random valid instructions
            let mut rng = StdRng::seed_from_u64(round);
            {
                let mut i = state.exestart+30;
                while i < state.exeend-10000*3 {
                    state.memory1[i as usize] = rng.gen_range(1..11);
                    i+=1;
                    if i >= state.exeend {break;}
                    state.memory1[i as usize] = rng.gen::<u16>().into();
                    i+=1;
                    if i >= state.exeend {break;}
                    state.memory1[i as usize] = rng.gen::<u16>().into();
                    i+=1;
                    if i >= state.exeend {break;}
                }
            }
            println!("round{}", round);
        }
        round+=1;

        // Wait a bit
        // sleep(Duration::from_millis(50));

    }
}
