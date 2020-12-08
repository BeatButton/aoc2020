const INPUT: &str = include_str!("input");

enum Op {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}

fn infinite(program: &Vec<Op>) -> (bool, isize) {
    let mut idx: isize = 0;
    let mut acc: isize = 0;
    let mut seen: Vec<bool> = vec![false; program.len()];
    loop {
        if idx as usize == program.len() {
            return (false, acc);
        }
        if seen[idx as usize] {
            return (true, acc);
        }
        seen[idx as usize] = true;
        match program[idx as usize] {
            Op::Acc(amt) => {
                acc += amt;
                idx += 1;
            }
            Op::Jmp(amt) => idx += amt,
            Op::Nop(_) => idx += 1,
        }
    }
}

fn swap(input: &mut Vec<Op>, idx: usize) {
    input[idx] = match input[idx] {
        Op::Acc(amt) => Op::Acc(amt),
        Op::Jmp(amt) => Op::Nop(amt),
        Op::Nop(amt) => Op::Jmp(amt),
    };
}

fn main() {
    let mut input: Vec<Op> = INPUT
        .lines()
        .map(|line| {
            let mut split = line.split(' ');
            let op = split.next().unwrap();
            let amt: isize = split.next().unwrap().parse().unwrap();
            match op {
                "acc" => Op::Acc(amt),
                "jmp" => Op::Jmp(amt),
                "nop" => Op::Nop(amt),
                _ => panic!("invalid opcode"),
            }
        })
        .collect();

    for i in 0..input.len() {
        swap(&mut input, i);
        let (inf, acc) = infinite(&input);
        if !inf {
            println!("{}", acc);
            break;
        }
        swap(&mut input, i);
    }
}
