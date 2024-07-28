#[cfg(test)]
mod test {
    use std::u32;

    use crate::cpu::{
        bus::Bus, memory::MemoryBuilder, Cpu, REGISTER_R10, REGISTER_RAX, REGISTER_RCX,
        REGISTER_RSI,
    };


    #[test]
    fn mov_88_test() {
        let mut memory: MemoryBuilder = MemoryBuilder::new(100);

        /* mov %al, %ch */
        memory.write8(0x88);
        memory.write8(0xc5);

        let bus = Bus::new(memory.generate());
        let mut cpu = Cpu::new(bus);
        cpu.registers[REGISTER_RAX] = 10;
        cpu.registers[REGISTER_RCX] = 0;

        cpu.boot();

        let as_bytes = cpu.registers[REGISTER_RAX].to_le_bytes();
        assert_eq!(as_bytes[0], 10);
    }

    #[test]
    fn mov_c7_test() {
        let mut memory: MemoryBuilder = MemoryBuilder::new(100);

        /* mov %al, %ch */
        memory.write8(0x48);
        memory.write8(0xc7);
        memory.write8(0xc0);
        memory.write32(0x1234);

        let bus = Bus::new(memory.generate());
        let mut cpu = Cpu::new(bus);
        cpu.boot();

        assert_eq!(cpu.registers[REGISTER_RAX], 0x1234);
    }

    #[test]
    fn write_32_to_64_memory_test() {
        let mut memory: MemoryBuilder = MemoryBuilder::new(100);

        /* mov $0x1111111111111111, %rax */
        memory.write8(0x48);
        memory.write8(0xb8);
        memory.write64(0x1111111111111111);

        /* mov $0x22222222, %eax */
        memory.write8(0xb8);
        memory.write32(0x22222222);

        let bus = Bus::new(memory.generate());
        let mut cpu = Cpu::new(bus);
        cpu.boot();

        assert_eq!(cpu.registers[REGISTER_RAX], 0x1111111122222222);
    }

    #[test]
    fn basic_low_byte_assign_test() {
        let mut memory: MemoryBuilder = MemoryBuilder::new(100);

        /* mov $0x1234, %rax */
        memory.write8(0x48);
        memory.write8(0xc7);
        memory.write8(0xc0);
        memory.write64(0x1234);

        /* mov $0x5678, %rcx */
        memory.write8(0x48);
        memory.write8(0xc7);
        memory.write8(0xc1);
        memory.write64(0x5678);

        /* mov %al, %cl */
        memory.write8(0x88);
        memory.write8(0xc1);

        let bus = Bus::new(memory.generate());
        let mut cpu = Cpu::new(bus);
        cpu.boot();

        assert_eq!(cpu.registers[REGISTER_RCX], 0x5634);
    }

    #[test]
    fn basic_high_byte_assign_test() {
        let mut memory: MemoryBuilder = MemoryBuilder::new(100);

        /* mov $0x1234, %rax */
        memory.write8(0x48);
        memory.write8(0xc7);
        memory.write8(0xc0);
        memory.write64(0x1234);

        /* mov $0x5678, %rcx */
        memory.write8(0x48);
        memory.write8(0xc7);
        memory.write8(0xc1);
        memory.write64(0x5678);

        /* mov %ah, %ch */
        memory.write8(0x88);
        memory.write8(0xe5);

        let bus = Bus::new(memory.generate());
        let mut cpu = Cpu::new(bus);
        cpu.boot();

        assert_eq!(cpu.registers[REGISTER_RCX], 0x1278);
    }
    
    #[test]
    fn basic_high_to_low_assign_test() {
        let mut memory: MemoryBuilder = MemoryBuilder::new(100);

        /* mov $0x1234, %rax */
        memory.write8(0x48);
        memory.write8(0xc7);
        memory.write8(0xc0);
        memory.write64(0x1234);

        /* mov %ah, %al */
        memory.write8(0x88);
        memory.write8(0xe0);

        let bus = Bus::new(memory.generate());
        let mut cpu = Cpu::new(bus);
        cpu.boot();

        assert_eq!(cpu.registers[REGISTER_RAX], 0x1212);
    }
    
    #[test]
    fn basic_low_to_high_assign_test() {
        let mut memory: MemoryBuilder = MemoryBuilder::new(100);

        /* mov $0x1234, %rax */
        memory.write8(0x48);
        memory.write8(0xc7);
        memory.write8(0xc0);
        memory.write64(0x1234);

        /* mov %ah, %al */
        memory.write8(0x88);
        memory.write8(0xc4);

        let bus = Bus::new(memory.generate());
        let mut cpu = Cpu::new(bus);
        cpu.boot();

        assert_eq!(cpu.registers[REGISTER_RAX], 0x3434);
    }
    

    /*#[test]
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
            println!(
                "RAX: {}, RCX: {}",
                cpu.registers[REGISTER_RAX], cpu.registers[REGISTER_RCX]
            )
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
            println!(
                "RAX: {}, RCX: {}, RSI: {}",
                cpu.registers[REGISTER_RAX],
                cpu.registers[REGISTER_RCX],
                cpu.registers[REGISTER_RSI]
            )
        });

        cpu.boot();

        assert_eq!(cpu.registers[REGISTER_RSI], 30);
    }*/
}
