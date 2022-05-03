use lazy_regex::*;
use std::collections::HashMap;
use std::collections::VecDeque;

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

#[derive(Debug)]
enum State {
    Q(usize),
    GARBAGE,
}

impl State {
    fn is_accept(&self) -> bool {
        match self {
            State::Q(num) => num == &2,
            State::GARBAGE => false,
        }
    }
}

type QState = usize;
type Symbol = usize;
type TransitionKey = (QState, Symbol);
type TransitionStep = (QState, Symbol, MoveDir);
type TransitionTuple = (TransitionKey, TransitionStep);

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
                self.state = transition.1;
                self.tape[self.ptr] = transition.1;
                let new_idx = self.ptr as i32 + transition.2.tape_ptr_move();
                if new_idx < 0 {
                    self.tape.push_front(3);
                    // the index of our element is now off by one because we prepended
                    self.ptr += 1;
                } else if new_idx == self.tape.len() as i32 {
                    self.tape.push_back(3);
                } else if new_idx > self.tape.len() as i32 {
                    panic!("Machine tape overflow!");
                }
                true
            }
            None => false
        }
    }
}

fn parse_transition(trans_str: &str) -> TransitionTuple {
    if regex_is_match!(r"(0+)1(0+)1(0+)1(0+)1(0{1,2})", trans_str) {
        let (_whole, l_q_from, l_symbol_read, l_q_to, l_symbol_wrote, l_move_dir) =
            regex_captures!(r"(0+)1(0+)1(0+)1(0+)1(0{1,2})", trans_str).unwrap();
        // symbol is its len(== number) - 1
        (
            (l_q_from.len(), l_symbol_read.len() - 1),
            (l_q_to.len(), l_symbol_wrote.len() - 1, l_move_dir.into()),
        )
    } else {
        panic!("{} doesnt represent a valid transition", trans_str)
    }
}

fn main() {
    
    /*
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 1 {
        panic!("Invalid number of arguments");
    }
    let tm_str = &args[0];
    */
    let TRANS_SEPARATOR = "11";

    let tm_str = "0100100010100";

    //let transitions: Vec<Transition> = tm_str.split(TRANS_SEPARATOR).map(|trans_str| trans_str.into()).collect();

    let transition_map: HashMap<TransitionKey, TransitionStep> = tm_str
        .split(TRANS_SEPARATOR)
        .map(|trans_str| parse_transition(trans_str))
        .collect();

    let mut machine = TuringM {
        tape: VecDeque::new(),
        ptr: 0,
        state: 1,
        transitions: &transition_map
    };

    println!("Turin world");

    println!("{:?}", transition_map);
}
