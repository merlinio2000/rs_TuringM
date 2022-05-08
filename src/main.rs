use lazy_regex::*;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fmt;

#[derive(Debug)]
enum MoveDir {
    LEFT,
    RIGHT,
}

impl MoveDir {
    fn tape_ptr_move(&self) -> i32 {
        match self {
            MoveDir::LEFT => -1,
            MoveDir::RIGHT => 1,
        }
    }
}

impl From<&str> for MoveDir {
    fn from(s: &str) -> MoveDir {
        if s == "0" {
            MoveDir::LEFT
        } else if s == "00" {
            MoveDir::RIGHT
        } else {
            panic!("unmatching string for move dir: {}", s)
        }
    }
}

impl fmt::Display for MoveDir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            MoveDir::LEFT => "L",
            MoveDir::RIGHT => "R"
        }
        )
    }
}

#[derive(Debug)]
enum State {
    Q(usize),
    GARBAGE,
}

impl State {
    fn is_accept(&self) -> bool {
        match self {
            State::Q(num) => *num == 2,
            State::GARBAGE => false,
        }
    }
}

type QState = usize;
type Symbol = usize;
type TransitionKey = (QState, Symbol);
type TransitionStep = (QState, Symbol, MoveDir);
type TransitionTuple = (TransitionKey, TransitionStep);

pub const EMPTY_WORD: Symbol = 3;
pub const TRANS_SEPARATOR: &str = "11";

#[derive(Debug)]
struct TuringM<'a> {
    tape: VecDeque<Symbol>,
    ptr: usize,
    state: QState,
    transitions: &'a HashMap<TransitionKey, TransitionStep>,
    step_cnt: usize
}

impl TuringM<'_> {
    fn step(&mut self) -> bool {
        match self.transitions.get(&(self.state, self.tape[self.ptr])) {
            Some(transition) => {
                println!("Pointer: {} ({}, {}) -> {:?}", self.ptr, self.state, self.tape[self.ptr], transition);
                println!("Tape: {:?}", self.tape);
                println!();
                self.state = transition.0;
                self.tape[self.ptr] = transition.1;
                let new_idx = self.ptr as i32 + transition.2.tape_ptr_move();
                if new_idx < 0 {
                    println!("prepending tape");
                    self.tape.push_front(EMPTY_WORD);
                    // the index of our element is now off by one because we prepended
                    self.ptr = (new_idx + 1) as usize;
                } else if new_idx == self.tape.len() as i32 {
                    println!("appending tape");
                    self.tape.push_back(EMPTY_WORD);
                    self.ptr = new_idx as usize;
                } else if new_idx > self.tape.len() as i32 {
                    panic!("Machine tape overflow!");
                } else {
                    self.ptr = new_idx as usize;
                }
                self.step_cnt += 1;
                true
            }
            None => {
                println!("Coulnt find transition for ({}, {})", self.state, self.tape[self.ptr]);
                false
            }
        }
    }
}


fn parse_transition(trans_str: &str) -> TransitionTuple {
    if regex_is_match!(r"(0+)1(0+)1(0+)1(0+)1(0{1,2})", trans_str) {
        let (_whole, l_q_from, l_symbol_read, l_q_to, l_symbol_wrote, l_move_dir) =
            regex_captures!(r"(0+)1(0+)1(0+)1(0+)1(0{1,2})", trans_str).unwrap();
        // len == symbol number / state number 
        (
            (l_q_from.len(), l_symbol_read.len()),
            (l_q_to.len(), l_symbol_wrote.len(), l_move_dir.into()),
        )
    } else {
        panic!("{} doesnt represent a valid transition", trans_str)
    }
}

fn parse_tape(tape_str: &str) -> VecDeque<Symbol> {
    tape_str.chars().map(|c| (c.to_digit(10).unwrap() as usize) + 1).collect()
}

fn main() {
    
    /*
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 1 {
        panic!("Invalid number of arguments");
    }
    let tape_str = &args[0];
    */
    let tape_str = "00010000";

    let tm_str = "01010100001001101000001010000100110100100010010011000100100010010011000101000010010110000100100001001011000001000010000010000101100001000010000010000010110000010100000101011000001000100000010100110000001000010000001000010011000000100000100001000001011000000101000000101001100001010000101001100000001001000000010001011000000010000100000001000101100000001010010100110001000100000001000101100001000001010000100";

    let transition_map: HashMap<TransitionKey, TransitionStep> = tm_str
        .split(TRANS_SEPARATOR)
        .map(|trans_str| parse_transition(trans_str))
        .collect();

    let mut machine = TuringM {
        tape: parse_tape(tape_str),
        ptr: 0,
        state: 1,
        transitions: &transition_map,
        step_cnt: 0
    };


    println!("Turin world");

    println!("{:?}", transition_map);


    while machine.step() {}
    println!("Machine ended in State {} after {} steps, acccepting={}", machine.state, machine.step_cnt, machine.state == 2);
    println!("Tape: {:?}", machine.tape);
}
