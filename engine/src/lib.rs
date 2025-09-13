//! BFDB is an optimising Brainfuck engine.

pub struct Engine {
    program: Vec<Instruction>,
    program_counter: usize,
    memory: Vec<u8>,
    memory_pointer: usize,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            program: Vec::new(),
            program_counter: 0,
            memory: vec![0],
            memory_pointer: 0,
        }
    }
}

pub enum Instruction {
    Left,
    Right,
    Increment,
    Decrement,
    Write,
    Read,
    LoopStart,
    LoopEnd,
}
