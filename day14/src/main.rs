const INPUT: &str = include_str!("input");

fn main() {
    let mut one_mask = 0_u64;
    let mut zero_mask = 2_u64.pow(35) - 1;
    let mut mem = vec![0_u64; 100_000];
    for line in INPUT.lines() {
        let mut iter = line.split(" = ");
        let instr = iter.next().unwrap();
        let value = iter.next().unwrap();
        if instr == "mask" {
            zero_mask = value
                .chars()
                .rev()
                .enumerate()
                .map(|(idx, ch)| if ch == '0' { 0 } else { 2_u64.pow(idx as u32) })
                .sum();

            one_mask = value
                .chars()
                .rev()
                .enumerate()
                .map(|(idx, ch)| if ch == '1' { 2_u64.pow(idx as u32) } else { 0 })
                .sum();
        } else {
            let addr = instr
                .chars()
                .skip(4)
                .take_while(|&ch| ch != ']')
                .collect::<String>()
                .parse::<usize>()
                .unwrap();
            let mut value = value.parse::<u64>().unwrap();
            value &= zero_mask;
            value |= one_mask;
            mem[addr] = value;
        }
    }
    println!("{}", mem.iter().sum::<u64>());
}
