#![allow(warnings)]

use cpu::{bus::Bus, memory::MemoryBuilder, Cpu};

mod cpu;
mod format;

fn main() {
    let mut memory = MemoryBuilder::new(100);
    format::parse("./assets/test.o")
        .into_iter()
        .for_each(|code| {
            print!("{} ", code);
            memory.write8(code)
        });

    let bus = Bus::new(memory.generate());
    let mut cpu = Cpu::new(bus);
    cpu.boot();
    println!("RAX: {}", cpu.registers[0]);
    println!("RCX: {}", cpu.registers[1]);

}
