use std::io::{self, BufRead};

#[derive(Debug)]
struct Machine {
    registers: [i64; 4],
}

impl Machine {
    fn new() -> Machine {
        Machine { registers: [0; 4] }
    }

    fn set_reg(&mut self, idx: usize, val: i64) {
        self.registers[idx] = val;
    }

    fn get_reg(&self, idx: usize) -> i64 {
        self.registers[idx]
    }
}

enum Operand {
    Register(usize),
    Immediate(i64),
}

enum BinaryOp {
    Add,
    Mul,
    Div,
    Mod,
    Eql,
}

fn parse_register_idx(s: &str) -> usize {
    match s {
        "w" => 0,
        "x" => 1,
        "y" => 2,
        "z" => 3,
        _ => panic!("Invalid register: {}", s),
    }
}

fn parse_operand(mut it: impl Iterator<Item = impl AsRef<str>>) -> Operand {
    let s = it.next().unwrap();
    if let Ok(imm) = s.as_ref().parse::<i64>() {
        Operand::Immediate(imm)
    } else {
        Operand::Register(parse_register_idx(s.as_ref()))
    }
}

fn parse_register(it: &mut impl Iterator<Item = impl AsRef<str>>) -> usize {
    match parse_operand(it) {
        Operand::Register(idx) => idx,
        _ => {
            panic!("Expected register");
        }
    }
}

fn parse_digit(it: &mut impl Iterator<Item = char>) -> i64 {
    it.next().unwrap() as i64 - '0' as i64
}

fn binary_op<'a>(
    machine: &mut Machine,
    mut it: impl Iterator<Item = &'a str>,
    operator: BinaryOp,
) -> Result<(), ()> {
    let dest_reg = parse_register(&mut it);
    let operand = parse_operand(&mut it);
    let op1 = machine.get_reg(dest_reg);
    let op2 = match operand {
        Operand::Register(idx) => machine.get_reg(idx),
        Operand::Immediate(val) => val,
    };
    let result = match operator {
        BinaryOp::Add => op1.checked_add(op2).ok_or(())?,
        BinaryOp::Mul => op1.checked_mul(op2).ok_or(())?,
        BinaryOp::Div => op1.checked_div(op2).ok_or(())?,
        BinaryOp::Mod => op1.checked_rem(op2).ok_or(())?,
        BinaryOp::Eql => {
            if op1 == op2 {
                1
            } else {
                0
            }
        }
    };
    machine.set_reg(dest_reg, result);
    Ok(())
}

fn eval(
    machine: &mut Machine,
    inst_ref: impl AsRef<str>,
    mut input_stream: impl Iterator<Item = char>,
) -> Result<(), ()> {
    let inst = inst_ref.as_ref();
    let mut tokenizer = inst.split_whitespace();
    match tokenizer.next().unwrap() {
        "inp" => {
            let reg = parse_register(&mut tokenizer);
            let imm = parse_digit(&mut input_stream);
            machine.set_reg(reg, imm);
        }
        "add" => binary_op(machine, tokenizer, BinaryOp::Add)?,
        "mul" => binary_op(machine, tokenizer, BinaryOp::Mul)?,
        "div" => binary_op(machine, tokenizer, BinaryOp::Div)?,
        "mod" => binary_op(machine, tokenizer, BinaryOp::Mod)?,
        "eql" => binary_op(machine, tokenizer, BinaryOp::Eql)?,
        _ => panic!("Invalid instruction: {}", inst),
    }

    Ok(())
}

fn run(
    mut machine: Machine,
    stream: Vec<impl AsRef<str>>,
    generator: impl Iterator<Item = impl AsRef<str>>,
) {
    for number_iter in generator {
        let number = number_iter.as_ref();
        println!("Using number: {}", number);

        let mut digit_iter = number.chars();
        let mut skip = false;

        for inst_ref in &stream {
            let inst = inst_ref.as_ref();
            let result = eval(&mut machine, inst, &mut digit_iter);
            if result.is_err() {
                skip = true;
                break;
            }
        }

        if skip {
            continue;
        }

        // Valid?
        println!("Registers: {:?}", machine);
        let valid = machine.get_reg(parse_register_idx("z"));
        if valid == 0 {
            println!("Found: {}", number);
        }
    }
}

struct NumberGenerator {
    number_str: String,
}

impl NumberGenerator {
    fn new() -> NumberGenerator {
        NumberGenerator {
            number_str: "99999999999999".to_string(),
        }
    }

    fn any_zeroes(v: i64) -> bool {
        v.to_string().chars().any(|c| c == '0')
    }

    fn advance(&mut self) {
        let mut v = self.number_str.parse::<i64>().unwrap();
        v -= 1;
        while Self::any_zeroes(v) {
            v -= 1;
        }
        self.number_str = v.to_string();
    }

    fn get(&self) -> String {
        self.number_str.clone()
    }
}

impl Iterator for NumberGenerator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.get();
        self.advance();
        Some(ret)
    }
}

fn main() {
    let generator = NumberGenerator::new();
    let machine = Machine::new();
    let stream = io::stdin()
        .lock()
        .lines()
        .map(|x| x.unwrap())
        .collect::<Vec<String>>();
    run(machine, stream, generator);
    println!("Hello, world!");
}
