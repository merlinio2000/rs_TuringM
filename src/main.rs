use lazy_regex::*;
use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug)]
enum MoveDir {
    LEFT, RIGHT
}

impl MoveDir {
    fn tape_ptr_move(&self) -> i32 {
        match self {
            MoveDir::LEFT => -1,
            MoveDir::RIGHT => 1
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
    Q(usize), GARBAGE
}

impl State {
    fn is_accept(&self) -> bool {
        match self {
            State::Q(num) => num == &2,
            State::GARBAGE => false
        }
    }
}



type QState = usize;
type Symbol = usize;


#[derive(Debug)]
struct TuringM {
    tape: VecDeque<Symbol>,
    ptr: usize,
    state: QState 
}

fn step(machine: &mut TuringM, transitions: &HashMap<(QState, Symbol), (QState, Symbol, MoveDir)>) -> bool {
    match transitions.get(&(machine.state, machine.tape[machine.ptr])) {
        Some(transition) => {
            machine.state = transition.1;
            machine.tape[machine.ptr] = transition.1;
            let newIdx = machine.ptr as i32 + transition.2.tape_ptr_move();
            if newIdx < 0 {
                machine.tape.push_front(3);
                machine.ptr += 1;
            } else if newIdx == machine.tape.len() as i32 {
                machine.tape.push_back(3);
            } else if newIdx > machine.tape.len() as i32 {
                panic!("Machine tape overflow!");
            }
            true
        }
        None => false
    }
    
}

fn parse_transition(trans_str: &str) -> ((QState, Symbol), (QState, Symbol, MoveDir)) {
    if regex_is_match!(r"(0+)1(0+)1(0+)1(0+)1(0{1,2})", trans_str) {
        let (_whole, l_q_from, l_symbol_read, l_q_to, l_symbol_wrote, l_move_dir) = regex_captures!(r"(0+)1(0+)1(0+)1(0+)1(0{1,2})", trans_str).unwrap();
        
        // symbol is its len(== number) - 1
        ((l_q_from.len(), l_symbol_read.len() - 1), 
            (l_q_to.len(), l_symbol_wrote.len() -1, l_move_dir.into()))
    } else {
        panic!("{} doesnt represent a valid transition", trans_str)
    }
}


fn main() {

    let args: Vec<String> = std::env::args().collect(); 
    if args.len() < 1 {
        panic!("Not enough args");
    }

    let TRANS_SEPARATOR = "11";

    let tm_str = "0100100010100";

    //let transitions: Vec<Transition> = tm_str.split(TRANS_SEPARATOR).map(|trans_str| trans_str.into()).collect();
    
    let mut transition_map: HashMap<(QState, Symbol), (QState, Symbol, MoveDir)> = tm_str.split(TRANS_SEPARATOR).map(|trans_str| {
        parse_transition(trans_str)
    }).collect();

    let mut machine = TuringM{tape: VecDeque::new(), ptr: 0, state: 1};

    

    

    

    println!("Turin world");

    println!("{:?}", transition_map);

}