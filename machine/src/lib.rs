pub struct Machine {
    pub acc: isize,
    pub idx: isize,
    pub seen: Vec<bool>,
    pub program: Vec<Op>,
}

pub enum Op {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}
pub enum Outcome {
    Running,
    Halted,
}

pub enum Error {
    InfiniteLoop,
}

type Result = std::result::Result<Outcome, Error>;

impl Machine {
    pub fn new<I, S>(input: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let program: Vec<Op> = input.into_iter().map(|s| Op::from(s.as_ref())).collect();
        let seen = vec![false; program.len()];
        Self {
            acc: 0,
            idx: 0,
            program,
            seen,
        }
    }

    pub fn run(&mut self) -> Result {
        loop {
            match self.step() {
                Ok(Outcome::Running) => {}
                otherwise => return otherwise,
            }
        }
    }

    pub fn step(&mut self) -> Result {
        let idx = self.idx as usize;
        if idx == self.program.len() {
            return Ok(Outcome::Halted);
        }
        if self.seen[idx] {
            return Err(Error::InfiniteLoop);
        }
        self.seen[idx] = true;
        match self.program[idx] {
            Op::Acc(amt) => {
                self.acc += amt;
                self.idx += 1;
            }
            Op::Jmp(offset) => {
                self.idx += offset;
            }
            Op::Nop(_) => self.idx += 1,
        }
        Ok(Outcome::Running)
    }

    pub fn reset(&mut self) {
        self.acc = 0;
        self.idx = 0;
        self.seen.iter_mut().for_each(|seen| *seen = false);
    }
}

impl From<&str> for Op {
    fn from(line: &str) -> Self {
        let mut line = line.split(' ');
        let code = line.next().expect("instruction line empty");
        let val: isize = line
            .next()
            .expect("instruction missing argument")
            .parse()
            .expect("argument not a valid number");
        match code {
            "acc" => Op::Acc(val),
            "jmp" => Op::Jmp(val),
            "nop" => Op::Nop(val),
            other => panic!("Unrecognized opcode {}", other),
        }
    }
}
