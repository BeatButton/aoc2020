use machine::{Machine, Op};

const INPUT: &str = include_str!("input");

fn swap(input: &mut Vec<Op>, idx: usize) {
    input[idx] = match input[idx] {
        Op::Acc(amt) => Op::Acc(amt),
        Op::Jmp(amt) => Op::Nop(amt),
        Op::Nop(amt) => Op::Jmp(amt),
    };
}

fn main() {
    let mut machine = Machine::new(INPUT.lines());

    for i in 0..machine.program.len() {
        swap(&mut machine.program, i);
        if let Err(machine::Error::InfiniteLoop) = machine.run() {
            println!("{}", machine.acc);
            break;
        }
        swap(&mut machine.program, i);
    }
}
