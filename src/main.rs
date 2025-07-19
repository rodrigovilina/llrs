use std::ops::ControlFlow;

#[derive(Clone, Debug)]
enum Op {
  Push(u8),
  Add,
  Sub,
  Mul,
  Print,
  PrintTop,
  Dup,
  Swap,
  Drop,
}

impl Op {
  fn opcode(&self) -> u8 {
    match self {
      Op::Push(_) => 0x01,
      Op::Add => 0x02,
      Op::Sub => 0x03,
      Op::Mul => 0x04,
      Op::Print => 0x05,
      Op::PrintTop => 0x06,
      Op::Dup => 0x07,
      Op::Swap => 0x08,
      Op::Drop => 0x09,
    }
  }

  fn to_bytecode(&self) -> Vec<u8> {
    match self {
      Op::Push(value) => vec![self.opcode(), *value],
      _ => vec![self.opcode()],
    }
  }

  fn from_parts(parts: &[&str]) -> Self {
    match parts {
      ["PUSH", value] => {
        let value: u8 = value.parse().expect("Expected an 8-bit number");
        Op::Push(value)
      },
      ["ADD"] => Op::Add,
      ["SUB"] => Op::Sub,
      ["MUL"] => Op::Mul,
      ["PRINT"] => Op::Print,
      ["PRINT_TOP"] => Op::PrintTop,
      ["DUP"] => Op::Dup,
      ["SWAP"] => Op::Swap,
      ["DROP"] => Op::Drop,
      _ => panic!("Unknown or malformed instruction: {parts:?}"),
    }
  }
}

#[derive(Debug)]
struct VM {
  stack: Vec<u8>,
  ip: usize,
  program: Vec<u8>,
}

impl VM {
  fn new(program: Vec<u8>) -> Self {
    Self {
      stack: vec![],
      ip: 0,
      program,
    }
  }

  fn run(&mut self) {
    while self.ip < self.program.len() {
      self.run_instruction();
    }
  }

  fn run_instruction(&mut self) {
    let opcode: u8 = self.program[self.ip];
    self.ip += 1;

    match opcode {
      0x01 => self.push(),
      0x02 => self.add(),
      0x03 => self.sub(),
      0x04 => self.mul(),
      0x05 => self.print(),
      0x06 => self.print_top(),
      0x07 => self.dup(),
      0x08 => self.swap(),
      0x09 => self.drop(),
      _ => panic!("Unknown opcode: {opcode}"),
    }
  }

  fn push(&mut self) {
    let value: u8 = self.program[self.ip];
    self.ip += 1;
    self.stack.push(value);
  }

  fn add(&mut self) {
    let b: u8 = self.stack.pop().expect("Stack underflow");
    let a: u8 = self.stack.pop().expect("Stack underflow");
    let result: u8 = a.wrapping_add(b);
    self.stack.push(result);
  }

  fn sub(&mut self) {
    let b: u8 = self.stack.pop().expect("Stack underflow");
    let a: u8 = self.stack.pop().expect("Stack underflow");
    let result: u8 = a.wrapping_sub(b);
    self.stack.push(result);
  }

  fn mul(&mut self) {
    let b: u8 = self.stack.pop().expect("Stack underflow");
    let a: u8 = self.stack.pop().expect("Stack underflow");
    let result: u8 = a.wrapping_mul(b);
    self.stack.push(result);
  }

  fn print(&mut self) {
    let value: u8 = self.stack.pop().expect("Stack underflow");
    println!("{value}");
  }

  fn print_top(&mut self) {
    let value: u8 = *self.stack.last().expect("Stack underflow");
    println!("{value}");
  }

  fn dup(&mut self) {
    let value: u8 = *self.stack.last().expect("Stack underflow");
    self.stack.push(value);
  }

  fn swap(&mut self) {
    let b: u8 = self.stack.pop().expect("Stack underflow");
    let a: u8 = self.stack.pop().expect("Stack underflow");
    self.stack.push(b);
    self.stack.push(a);
  }

  fn drop(&mut self) {
    self.stack.pop().expect("Stack underflow");
  }
}

fn assemble(source: &str) -> Vec<u8> {
  let mut bytecode: Vec<u8> = vec![];
  for line in source.lines() {
    if let ControlFlow::Break(_) = assemble_line(&mut bytecode, line) {
      continue;
    }
  }

  bytecode
}

fn assemble_line(bytecode: &mut Vec<u8>, line: &str) -> ControlFlow<()> {
  let parts: Vec<&str> = line.split_whitespace().collect();

  if parts.is_empty() {
    return ControlFlow::Break(());
  }

  let op: Op = Op::from_parts(&parts);
  bytecode.extend(op.to_bytecode());

  ControlFlow::Continue(())
}

fn main() {
  let source: &'static str = r#"
PUSH 5
PRINT_TOP
PUSH 10
PRINT_TOP
ADD
PRINT_TOP
PUSH 1
SUB
PRINT_TOP
DUP
ADD
PRINT_TOP
PUSH 7
PRINT_TOP
SWAP
PRINT_TOP
DROP
PUSH 20
PUSH 20
MUL
PRINT
"#;

  let program: Vec<u8> = assemble(source);
  let mut vm: VM = VM::new(program);

  vm.run();

  println!("Stack after execution: {:?}", vm.stack)
}
