mod avr_interpreter;
mod ps_utils;

use std::collections::{HashMap, VecDeque};
use avr_interpreter::*;
use ps_utils::*;

fn main() {
    let mut state_queue = VecDeque::new();
    let mut visited_states = HashMap::new();

    // Start with initial state of the processor
    let initial_state = ProcessorState::new();

    // Add initial state to the queue and mark it as visited
    state_queue.push_back(initial_state.clone());
    visited_states.insert(initial_state.clone(), 0);

    // Keep track of the highest survival chances so far
    let mut highest_chances = initial_state.clone();
    let mut highest_chances_count = count_possibilities(&initial_state);

    while let Some(state) = state_queue.pop_front() {
        // Execute all possible instruction changes from current state
        for next_state in execute_memory_region(&state) {
            // Skip if already visited
            if visited_states.contains_key(&next_state) {
                continue;
            }

            // Calculate number of possibilities for next state
            let next_state_count = count_possibilities(&next_state);

            // Compare current state with next state
            let result = compare_states(&state, &next_state);

            // Update highest survival chances if necessary
            if next_state_count > highest_chances_count {
                highest_chances = next_state.clone();
                highest_chances_count = next_state_count;
            }

            // Add next state to queue and mark as visited
            state_queue.push_back(next_state.clone());
            visited_states.insert(next_state.clone(), result);
        }
    }

    // Print the highest survival chances found
    println!("Highest survival chances: {:?}", highest_chances);
}
