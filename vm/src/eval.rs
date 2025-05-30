use crate::*;
use clearscreen::clear;
use colored::*;
use std::{mem::size_of, thread::sleep, time::Duration};

impl RukaVM {
    pub fn new(program: Vec<Instruction>) -> Self {
        println!("Welcome to Ruka VM!");
        println!("{} Bytes free", size_of::<[f64; MEMORY_SIZE]>());

        RukaVM {
            program,
            instruction: Instruction::Nop,
            memory: [0.0; MEMORY_SIZE],
            call: Vec::new(),
            stack: Vec::new(),
            pc: 0.0,
            ar: 0.0,
            dr: 0.0,
            cr: 0.0,
            ba: 0.0,
            sp: 0.0,
        }
    }

    pub fn start(&mut self) -> Option<()> {
        loop {
            self.instruction = self.program.get(self.pc as usize)?.clone();
            match self.instruction {
                Instruction::Mov(reg, val) => {
                    let val = self.get_operand(val);
                    let reg = self.get_register(reg);
                    *reg = val
                }
                Instruction::Add(dst, src) => {
                    let operand = self.get_operand(src);
                    let reg = self.get_register(dst);
                    *reg = *reg + operand
                }
                Instruction::Mul(dst, src) => {
                    let operand = self.get_operand(src);
                    let reg = self.get_register(dst);
                    *reg = *reg * operand
                }
                Instruction::Neg(reg) => {
                    let reg = self.get_register(reg);
                    *reg = -*reg
                }
                Instruction::Inv(reg) => {
                    let reg = self.get_register(reg);
                    *reg = 1.0 / *reg;
                }
                Instruction::Eql(dst, src) => {
                    let operand = self.get_operand(src);
                    let reg = self.get_register(dst);
                    *reg = if *reg == operand { 1.0 } else { 0.0 }
                }
                Instruction::Les(dst, src) => {
                    let operand = self.get_operand(src);
                    let reg = self.get_register(dst);
                    *reg = if *reg < operand { 1.0 } else { 0.0 }
                }
                Instruction::Nor(dst, src) => {
                    let operand = self.get_operand(src);
                    let reg = self.get_register(dst);
                    let result = !(*reg != 0.0 || operand != 0.0);
                    *reg = if result { 1.0 } else { 0.0 }
                }
                Instruction::Jmp(cond, addr) => {
                    let cond = self.get_operand(cond);
                    let addr = self.get_operand(addr);
                    if cond != 0.0 {
                        self.pc = addr;
                        continue;
                    }
                }
                Instruction::Cal(addr) => {
                    let addr = self.get_operand(addr);
                    self.call.push(self.pc);
                    self.pc = addr;
                    continue;
                }
                Instruction::Ret => self.pc = self.call.pop()?,
                Instruction::Lda(reg, addr) => {
                    let addr = self.get_operand(addr);
                    let val = self.memory.get(addr as usize)?.clone();
                    let reg = self.get_register(reg);
                    *reg = val
                }
                Instruction::Sta(addr, val) => {
                    let addr = self.get_operand(addr);
                    let val = self.get_operand(val);
                    let addr = self.memory.get_mut(addr as usize)?;
                    *addr = val;
                }
                Instruction::Psh(val) => {
                    let val = self.get_operand(val);
                    self.stack.push(val);
                    self.sp += 1.0;
                }
                Instruction::Pop(reg) => {
                    let val = self.stack.pop()?;
                    let reg = self.get_register(reg);
                    *reg = val;
                    self.sp -= 1.0;
                }
                Instruction::Nop => {}
                Instruction::Hlt => break,
            }
            self.dump();
            self.pc += 1.0;
        }
        Some(())
    }

    pub fn returns(&mut self, mode: BasedMode) -> Option<f64> {
        match mode {
            BasedMode::Stack => self.stack.pop(),
            BasedMode::Register => Some(self.ar),
        }
    }

    fn dump(&self) {
        macro_rules! view {
            ($val: expr) => {{
                let formatted = format!("{:08}", $val);
                if $val != 0.0 {
                    formatted.bold()
                } else {
                    formatted.normal()
                }
            }};
        }

        clear().unwrap();
        println!("# Ruka VM");

        println!("Instruction: {:?}", self.instruction);

        println!("Registers:");
        println!(" PC: {}  AR: {}", view!(self.pc), view!(self.ar));
        println!(" DR: {}  CR: {}", view!(self.dr), view!(self.cr));
        println!(" BA: {}  SP: {}", view!(self.ba), view!(self.sp));

        println!("Call Stack:");
        for (i, val) in self.call.iter().enumerate() {
            println!(" {}: {}", i, view!(*val));
        }

        println!("Stack Area:");
        for (i, val) in self.stack.iter().enumerate() {
            println!(" {}: {}", i, view!(*val));
        }

        println!("Memory Area:");
        for (i, vals) in self.memory.chunks(8).enumerate() {
            let i = i * 8;
            print!(" {i:02} ~ {:02}: ", i + 7);
            for val in vals {
                print!("{} ", view!(*val));
            }
            println!()
        }

        let free = self.memory.iter().filter(|x| **x == 0.0).count() * 8;
        let used = self.memory.len() * 8 - free;
        println!("{free} Bytes free, {used} Bytes used");

        sleep(Duration::from_secs_f64(0.3));
    }

    fn get_register(&mut self, register: Register) -> &mut f64 {
        match register {
            Register::Pc => &mut self.pc,
            Register::Ar => &mut self.ar,
            Register::Dr => &mut self.dr,
            Register::Cr => &mut self.cr,
            Register::Ba => &mut self.ba,
            Register::Sp => &mut self.sp,
        }
    }

    fn get_operand(&mut self, operand: Operand) -> f64 {
        match operand {
            Operand::Literal(value) => value,
            Operand::Register(register) => self.get_register(register).clone(),
        }
    }
}
