
#[cfg(test)]
mod test {
    use crate::cpu::{bus::Bus, memory::MemoryBuilder, Cpu, REGISTER_R10, REGISTER_R9, REGISTER_RAX, REGISTER_RCX, REGISTER_RSI};

    #[test]
    fn rex_1() {
        let mut memory = MemoryBuilder::new(100);
        /* mov $16, %rax */
        memory.write8(0x48);
        memory.write8(0xB8);
        memory.write64(0x10);
    
        /* mov $5, %rcx */
        memory.write8(0x48);
        memory.write8(0xB9);
        memory.write64(0x05);
    
        /* add %rax, %rcx */
        memory.write8(0x48);
        memory.write8(0x01);
        memory.write8(0xC1);
    
        let bus = Bus::new(memory.generate());
        let mut cpu = Cpu::new(bus);
        cpu.add_hook(|cpu| {
            println!("RAX: {}, RCX: {}", cpu.registers[REGISTER_RAX], cpu.registers[REGISTER_RCX])
        });

        cpu.boot();
        
        assert_eq!(cpu.registers[REGISTER_RAX], 16);
        assert_eq!(cpu.registers[REGISTER_RCX], 21);
    }

    #[test]
    fn rex_2() {
        let mut memory = MemoryBuilder::new(100);
        /* mov $16, %r10d */
        memory.write8(0x41);
        memory.write8(0xB8);
        memory.write64(0x10);
    
        let bus = Bus::new(memory.generate());
        let mut cpu = Cpu::new(bus);
        cpu.boot();
        
        assert_eq!(cpu.registers[8], 0x10);
    }

    #[test]
    fn max_u64_check() {
        let mut memory = MemoryBuilder::new(100);
        /* mov $16, %r10d */
        memory.write8(0x49);
        memory.write8(0xC7);
        memory.write8(0xC2);
        memory.write64(u64::MAX);
    
        let bus = Bus::new(memory.generate());
        let mut cpu = Cpu::new(bus);
        cpu.boot();
        
        assert_eq!(cpu.registers[REGISTER_R10], u64::MAX);
    }

    #[test]
    fn max_u64_check_2() {
        let mut memory: MemoryBuilder = MemoryBuilder::new(100);
        /* mov $18446744073709551615, %r10d */
        memory.write8(0x41);
        memory.write8(0xBA);
        memory.write64(u64::MAX);

        /* mov $0, %r10d */
        memory.write8(0x41);
        memory.write8(0xBA);
        memory.write64(0);
    
        let bus = Bus::new(memory.generate());
        let mut cpu = Cpu::new(bus);
        cpu.boot();
        
        assert_eq!(cpu.registers[REGISTER_R10], 0);
    }

    #[test]
    fn calculation_test() {
        let mut memory: MemoryBuilder = MemoryBuilder::new(100);
        /* mov $10, %rax */
        memory.write8(0x48);
        memory.write8(0xc7);
        memory.write8(0xc0);
        memory.write64(10);

        /* mov $20, %rcx */
        memory.write8(0x48);
        memory.write8(0xc7);
        memory.write8(0xc1);
        memory.write64(20);

        /* add %rcx, %rax */
        memory.write8(0x48);
        memory.write8(0x01);
        memory.write8(0xc8);

        /* mov %rax, %rsi */
        memory.write8(0x48);
        memory.write8(0x89);
        memory.write8(0xc6);
    
        let bus = Bus::new(memory.generate());
        let mut cpu = Cpu::new(bus);

        cpu.add_hook(|cpu| {
            println!("RAX: {}, RCX: {}, RSI: {}", cpu.registers[REGISTER_RAX], cpu.registers[REGISTER_RCX], cpu.registers[REGISTER_RSI])
        });

        cpu.boot();
        
        assert_eq!(cpu.registers[REGISTER_RSI], 30);
    }


/* mov $10, %rax
mov $20, %rcx
add %rcx, %rax
mov %rax, %rsi

add %rax, %rcx

mov %eax, %esi

mov $10, %rax
mov $0xf, (%rax)

*/
    #[test]
    fn add_test() {
        let mut memory: MemoryBuilder = MemoryBuilder::new(100);
        /* mov $10, %rax */
        memory.write8(0x48);
        memory.write8(0xc7);
        memory.write8(0xc0);
        memory.write64(10);

        /* mov $20, %rcx */
        memory.write8(0x48);
        memory.write8(0xc7);
        memory.write8(0xc1);
        memory.write64(20);

        /* add %rcx, %rax */
        memory.write8(0x48);
        memory.write8(0x01);
        memory.write8(0xc8);
    
        let bus = Bus::new(memory.generate());
        let mut cpu = Cpu::new(bus);

        cpu.add_hook(|cpu| {
            println!("RAX: {}, RCX: {}", cpu.registers[REGISTER_RAX], cpu.registers[REGISTER_RCX])
        });
        cpu.boot();
        
        assert_eq!(cpu.registers[REGISTER_RAX], 30);
        assert_eq!(cpu.registers[REGISTER_RCX], 20);
    }
}