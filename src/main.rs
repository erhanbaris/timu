use cpu::{bus::Bus, memory::MemoryBuilder, Cpu};

mod cpu;

fn main() {
    let mut memory = MemoryBuilder::new(100);
    /*memory.write8(0x48);
    memory.write8(0xB8);
    memory.write64(0x10);

    memory.write8(0x48);
    memory.write8(0xB9);
    memory.write64(0x05);

    memory.write8(0x01);
    memory.write8(0xC8);*/

    memory.write8(0x48);
    memory.write8(0xC7);
    memory.write8(0xC0);
    memory.write8(0x01);
    memory.write8(0x00);
    memory.write8(0x00);
    memory.write8(0x00);
    memory.write8(0x48);
    memory.write8(0xC7);
    memory.write8(0xC3);
    memory.write8(0x02);
    memory.write8(0x00);
    memory.write8(0x00);
    memory.write8(0x00);
    memory.write8(0x48);
    memory.write8(0x01);
    memory.write8(0xD8);
    memory.write8(0x48);
    memory.write8(0x01);
    memory.write8(0xC8);
    memory.write8(0x90);
    // memory.write8(0xF4);

    let bus = Bus::new(memory.generate());
    let mut cpu = Cpu::new(bus);
    cpu.boot();
    cpu.dump();
    println!("RAX: {}", cpu.registers[0]);
    println!("RCX: {}", cpu.registers[1]);
}
