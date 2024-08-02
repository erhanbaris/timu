#[cfg(test)]
mod test {
    use std::u32;

    use crate::cpu::{bus::Bus, memory::MemoryBuilder, Cpu, REGISTER_RAX, REGISTER_RCX};

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

        /* add %rcx, %rax */
        memory.write8(0x48);
        memory.write8(0x01);
        memory.write8(0xc8);
        memory.write8(0x90);

        let bus = Bus::new(memory.generate());
        let mut cpu = Cpu::new(bus);
        cpu.registers[REGISTER_RAX] = 10;
        cpu.registers[REGISTER_RCX] = 20;

        cpu.add_hook(|cpu| {
            println!(
                "RAX: {}, RCX: {}",
                cpu.registers[REGISTER_RAX], cpu.registers[REGISTER_RCX]
            )
        });
        cpu.boot();

        assert_eq!(cpu.registers[REGISTER_RAX], 30);
        assert_eq!(cpu.registers[REGISTER_RCX], 20);
    }

    #[test]
    fn add_00_test() {
        let mut memory = MemoryBuilder::new(100);

        /* add %cl, %al */
        memory.write8(0x00);
        memory.write8(0xc8);
        memory.write8(0x90);

        let bus = Bus::new(memory.generate());
        let mut cpu = Cpu::new(bus);
        cpu.registers[REGISTER_RAX] = 0xCCCC_CCCC_CCCC_CCBB;
        cpu.registers[REGISTER_RCX] = 0x1111_1111_1111_1111;

        cpu.add_hook(|cpu| {
            println!(
                "RAX: {}, RCX: {}",
                cpu.registers[REGISTER_RAX], cpu.registers[REGISTER_RCX]
            )
        });
        //cpu.boot();

        //assert_eq!(cpu.registers[REGISTER_RAX], 0xCCCC_CCCC_CCCC_CCCC);
        //assert_eq!(cpu.registers[REGISTER_RCX], 0x1111_1111_1111_1111);

        
        let mut memory: MemoryBuilder = MemoryBuilder::new(100);

        /* add %cl, %ah */
        memory.write8(0x00);
        memory.write8(0xcc);
        memory.write8(0x90);

        let bus = Bus::new(memory.generate());
        let mut cpu = Cpu::new(bus);
        cpu.registers[REGISTER_RAX] = 0xCCCC_CCCC_CCCC_BBCC;
        cpu.registers[REGISTER_RCX] = 0x1111_1111_1111_1111;

        cpu.add_hook(|cpu| {
            println!(
                "RAX: {}, RCX: {}",
                cpu.registers[REGISTER_RAX], cpu.registers[REGISTER_RCX]
            )
        });
        cpu.boot();

        assert_eq!(cpu.registers[REGISTER_RAX], 0xCCCC_CCCC_CCCC_CCCC);
        assert_eq!(cpu.registers[REGISTER_RCX], 0x1111_1111_1111_1111);
    }

    #[test]
    fn add_01_test() {
        let mut memory: MemoryBuilder = MemoryBuilder::new(100);

        /* add %rcx, %rax */
        memory.write8(0x48);
        memory.write8(0x01);
        memory.write8(0xc8);
        memory.write8(0x90);

        let bus = Bus::new(memory.generate());
        let mut cpu = Cpu::new(bus);
        cpu.registers[REGISTER_RAX] = 10;
        cpu.registers[REGISTER_RCX] = 5;

        cpu.add_hook(|cpu| {
            println!(
                "RAX: {}, RCX: {}",
                cpu.registers[REGISTER_RAX], cpu.registers[REGISTER_RCX]
            )
        });
        cpu.boot();

        assert_eq!(cpu.registers[REGISTER_RAX], 15);
        assert_eq!(cpu.registers[REGISTER_RCX], 5);
    }
    
    fn add_01_test_2() {
        let mut memory: MemoryBuilder = MemoryBuilder::new(100);
        
        /* addl %eax, %r9d */
        memory.write8(0x41);
        memory.write8(0x01);
        memory.write8(0xc1);

        /* addl %eax, %r9d */
        memory.write8(0x41);
        memory.write8(0x01);
        memory.write8(0xc1);
        memory.write8(0x90);

        let bus = Bus::new(memory.generate());
        let mut cpu = Cpu::new(bus);
        cpu.registers[REGISTER_RAX] = 10;
        cpu.registers[REGISTER_RCX] = 5;

        cpu.add_hook(|cpu| {
            println!(
                "RAX: {}, RCX: {}",
                cpu.registers[REGISTER_RAX], cpu.registers[REGISTER_RCX]
            )
        });
        cpu.boot();

        assert_eq!(cpu.registers[REGISTER_RAX], 15);
        assert_eq!(cpu.registers[REGISTER_RCX], 5);
    }

    #[test]
    fn add_01_test_3() {
        let mut memory: MemoryBuilder = MemoryBuilder::new(100);

        /* add %eax, 10 */
        memory.write8(0x01);
        memory.write8(0x04);
        memory.write8(0x25);
        memory.write32(10);
        memory.write8(0x90);

        let bus = Bus::new(memory.generate());
        let mut cpu = Cpu::new(bus);
        cpu.registers[REGISTER_RAX] = 5;

        cpu.add_hook(|cpu| {
            println!("RAX: {}", cpu.registers[REGISTER_RAX])
        });
        cpu.boot();

        assert_eq!(cpu.bus.read64(10), 5);
        assert_eq!(cpu.registers[REGISTER_RAX], 5);
    }

    #[test]
    fn add_04_test() {
        let mut memory: MemoryBuilder = MemoryBuilder::new(100);

        /* mov $0xFF00, %ecx */
        memory.write8(0xb9);
        memory.write32(0x0000_FF00);

        /* add $0x00FF, %ecx */
        memory.write8(0x81);
        memory.write8(0xC1);
        memory.write32(0x0000_00FF);
        memory.write8(0x90);

        let bus = Bus::new(memory.generate());
        let mut cpu = Cpu::new(bus);

        cpu.add_hook(|cpu| {
            println!("RCX: {}", cpu.registers[REGISTER_RCX])
        });

        cpu.boot();

        assert_eq!(cpu.registers[REGISTER_RCX], 0x0000_FFFF);
    }

    #[test]
    fn add_81_test() {
        let mut memory: MemoryBuilder = MemoryBuilder::new(100);

        /* add $0xfffffff, %ecx */
        memory.write8(0x81);
        memory.write8(0xc1);
        memory.write32(u32::MAX);
        memory.write8(0x90);

        let bus = Bus::new(memory.generate());
        let mut cpu = Cpu::new(bus);

        cpu.boot();

        assert_eq!(cpu.registers[REGISTER_RCX], u32::MAX as u64);

        let mut memory: MemoryBuilder = MemoryBuilder::new(100);

        /* add $0xfffffffff, %rcx */
        memory.write8(0x48);
        memory.write8(0x81);
        memory.write8(0xc1);
        memory.write64(u64::MAX);
        memory.write8(0x90);

        let bus = Bus::new(memory.generate());
        let mut cpu = Cpu::new(bus);

        cpu.boot();

        assert_eq!(cpu.registers[REGISTER_RCX], u64::MAX as u64);
    }
    

    #[test]
    fn add_03_test() {
        let mut memory: MemoryBuilder = MemoryBuilder::new(100);

        /* add (%RAX), %RCX */
        memory.write8(0x48);
        memory.write8(0x03);
        memory.write8(0x08);
        memory.write8(0x90); // Nop
        memory.write64(2000);
        memory.write8(0x90);

        let bus = Bus::new(memory.generate());
        let mut cpu = Cpu::new(bus);

        cpu.registers[REGISTER_RAX] = 4; // Pointer
        cpu.registers[REGISTER_RCX] = 24; 

        cpu.boot();

        assert_eq!(cpu.registers[REGISTER_RCX], 2024);
    }
    
    #[test]
    fn add_83_test() {
        let mut memory: MemoryBuilder = MemoryBuilder::new(100);

        /* add $10, %rax */
        memory.write8(0x48);
        memory.write8(0x83);
        memory.write8(0xc0);
        memory.write8(0x0a);
        memory.write8(0x90);

        let bus = Bus::new(memory.generate());
        let mut cpu = Cpu::new(bus);

        cpu.boot();

        assert_eq!(cpu.registers[REGISTER_RAX], 10);
    }

    #[test]
    fn add_05_test() {
        let mut memory: MemoryBuilder = MemoryBuilder::new(100);

        /* add $0xffff, %eax */
        memory.write8(0x05);
        memory.write32(0xffff);
        memory.write8(0x90);

        let bus = Bus::new(memory.generate());
        let mut cpu = Cpu::new(bus);
        cpu.boot();

        assert_eq!(cpu.registers[REGISTER_RAX], 0xffff);

        let mut memory: MemoryBuilder = MemoryBuilder::new(100);
        /* add $0xfffffff, %rax */
        memory.write8(0x48);
        memory.write8(0x05);
        memory.write64(0xfffffff);
        memory.write8(0x90);

        let bus = Bus::new(memory.generate());
        let mut cpu = Cpu::new(bus);
        cpu.boot();

        assert_eq!(cpu.registers[REGISTER_RAX], 0xfffffff);

        let mut memory: MemoryBuilder = MemoryBuilder::new(100);
        /* add $0xf000ffff, %ax */
        memory.write8(0x66);
        memory.write8(0x05);
        memory.write16(0xffff);
        memory.write8(0x90);

        let bus = Bus::new(memory.generate());
        let mut cpu = Cpu::new(bus);
        cpu.boot();

        assert_eq!(cpu.registers[REGISTER_RAX], 0xffff);
    }

    //TODO: finish
    fn calculate_dynamic_address_and_add() {
        let mut memory: MemoryBuilder = MemoryBuilder::new(100);

        /* add 0x10(%rax,%rbx,2), %ecx */
        memory.write8(0x03);
        memory.write8(0x4C);
        memory.write8(0x58);
        memory.write8(0x10);
        memory.write8(0x90);

        let bus = Bus::new(memory.generate());
        let mut cpu = Cpu::new(bus);
        cpu.boot();

        //assert_eq!(cpu.registers[REGISTER_RAX], 0xffff);
    }
}

/*
    mov $0, %rcx
    add $0x2, %cl // 0x2 2
    add $0x5, %ch // 0x502 1282
*/

//0xa00 2560
//0xa14 2580
