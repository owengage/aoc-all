use std::{
    fs::File,
    io::{BufRead, BufReader},
};

enum Ins {
    Noop,
    AddX(i64),
}

struct Cpu {
    x: i64,
    cycle: i64,
    code: Vec<Ins>,
    ins_end: i64,
}

impl Cpu {
    fn new(mut code: Vec<Ins>) -> Self {
        // We pop instructions off so need to reverse order.
        code.reverse();

        let ins_end = match code.last().unwrap() {
            Ins::Noop => 1,
            Ins::AddX(_) => 2,
        };

        Self {
            x: 1,
            cycle: 0,
            code,
            ins_end,
        }
    }

    fn tick(&mut self) {
        self.cycle += 1;
    }

    fn tock(&mut self) -> Option<()> {
        if self.cycle == self.ins_end {
            match self.code.pop()? {
                Ins::Noop => {}
                Ins::AddX(delta) => self.x += delta,
            }

            self.ins_end += match self.code.last()? {
                Ins::Noop => 1,
                Ins::AddX(_) => 2,
            };
        }

        Some(())
    }
}

fn parse(r: impl BufRead) -> Vec<Ins> {
    r.lines()
        .flatten()
        .map(|line| {
            if line == "noop" {
                Ins::Noop
            } else {
                let val = line.strip_prefix("addx ").unwrap();
                Ins::AddX(val.parse().unwrap())
            }
        })
        .collect()
}

fn main() {
    let ins = parse(BufReader::new(File::open("input.txt").unwrap()));

    let mut cpu = Cpu::new(ins);

    let mut signal_strength = 0;

    loop {
        if (cpu.x - (cpu.cycle % 40)).abs() <= 1 {
            print!("#")
        } else {
            print!(".")
        }

        cpu.tick();

        if (cpu.cycle - 20) % 40 == 0 {
            let str = cpu.cycle as i64 * cpu.x;
            signal_strength += str;
        }

        if cpu.cycle % 40 == 0 {
            println!();
        }

        if cpu.tock().is_none() {
            break;
        }
    }

    dbg!(signal_strength);
}
