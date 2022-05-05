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
        // symbol is its len(== number) - 1
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
    let tm_str = &args[0];
    */
    let tape_str = "0001000";

    let tm_str = "010101000010011010000010100001001101001000100100110001001000100100110001010000100101100001001000010010110000010000100000100001011000010000100000100000101100000101000001010110000010001000000101001100000010000100000010000100110000001000001000010000010110000001010000001010011000010100001010011000000010010000000100010110000000100001000000010001011000000010100101001100010001000100010";

    //let transitions: Vec<Transition> = tm_str.split(TRANS_SEPARATOR).map(|trans_str| trans_str.into()).collect();

    let transition_map: HashMap<TransitionKey, TransitionStep> = tm_str
        .split(TRANS_SEPARATOR)
        .map(|trans_str| parse_transition(trans_str))
        .collect();

    let mut machine = TuringM {
        tape: parse_tape(tape_str),
        ptr: 0,
        state: 1,
        transitions: &transition_map
    };


    println!("Turin world");

    println!("{:?}", transition_map);


    while machine.step() {}
    println!("Machine ended in State {}", machine.state);
    println!("Tape: {:?}", machine.tape);
}
