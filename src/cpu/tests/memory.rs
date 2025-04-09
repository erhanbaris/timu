#[cfg(test)]
mod test {

    use crate::cpu::{bus::Bus, memory::Memory};
    #[test]
    fn memory_empty() {
        let memory = Memory::new(1024);

        assert_eq!(memory.len(), 1024);

        for index in 0..1024 {
            assert_eq!(memory.read8(index), 0x0);
        }

        for index in 0..(1024 / 2) {
            assert_eq!(memory.read16(index), 0x0);
        }

        for index in 0..(1024 / 4) {
            assert_eq!(memory.read32(index), 0x0);
        }

        for index in 0..(1024 / 8) {
            assert_eq!(memory.read64(index), 0x0);
        }
    }

    #[test]
    fn read_write_test() {
        let mut memory = Memory::new(1024);

        assert_eq!(memory.len(), 1024);
        memory.write8(0, 0x0f);
        assert_eq!(memory.read8(0), 0x0f);

        memory.write16(0, 0x0ff0);
        assert_eq!(memory.read16(0), 0x0ff0);

        memory.write32(0, 0x1fff_1fff);
        assert_eq!(memory.read32(0), 0x1fff_1fff);

        memory.write64(0, 0x0ff1_0ff1_0ff1_0ff1);
        assert_eq!(memory.read64(0), 0x0ff1_0ff1_0ff1_0ff1);

        memory.write64(0, 0x1fff_2fff_3fff_4fff);
        assert_eq!(memory.read8(0), 0xff);
        assert_eq!(memory.read16(0), 0x4fff);
        assert_eq!(memory.read32(0), 0x3fff_4fff);
        assert_eq!(memory.read64(0), 0x1fff_2fff_3fff_4fff);

        assert_eq!(memory.read16(0), 0x4fff);
        assert_eq!(memory.read16(2), 0x3fff);
        assert_eq!(memory.read16(4), 0x2fff);
        assert_eq!(memory.read16(6), 0x1fff);
    }

    #[test]
    fn little_endian_test() {
        let mut memory = Memory::new(1024);

        assert_eq!(memory.len(), 1024);
        memory.write64(0, 0x1234_5678_9012_3456);
        assert_eq!(memory.read64(0), 0x1234_5678_9012_3456);

        assert_eq!(memory.read8(0), 0x56);
        assert_eq!(memory.read8(1), 0x34);
        assert_eq!(memory.read8(2), 0x12);
        assert_eq!(memory.read8(3), 0x90);
        assert_eq!(memory.read8(4), 0x78);
        assert_eq!(memory.read8(5), 0x56);
        assert_eq!(memory.read8(6), 0x34);
        assert_eq!(memory.read8(7), 0x12);
    }

    #[test]
    fn bus_test() {
        let mut memory = Memory::new(1024);
        memory.write8(0, 0x10);
        memory.write8(1, 0x20);
        memory.write8(2, 0x30);
        memory.write8(3, 0x40);
        memory.write8(4, 0x50);
        memory.write8(5, 0x60);
        memory.write8(6, 0x70);
        memory.write8(7, 0x80);

        let mut bus = Bus::new(memory);
        assert_eq!(bus.read8(0), 0x10);
        assert_eq!(bus.read8(1), 0x20);
        assert_eq!(bus.read8(2), 0x30);
        assert_eq!(bus.read8(3), 0x40);
        assert_eq!(bus.read8(4), 0x50);
        assert_eq!(bus.read8(5), 0x60);
        assert_eq!(bus.read8(6), 0x70);
        assert_eq!(bus.read8(7), 0x80);

        bus.write8(0, 0xff);
        assert_eq!(bus.read8(0), 0xff);
        assert_eq!(bus.len(), 1024);
    }

    #[test]
    fn bit_test() {
        let mut memory = Memory::new(1024);
        memory.write8(0, 0x10);
        memory.write8(1, 0x20);
        memory.write8(2, 0x30);
        memory.write8(3, 0x40);
        memory.write8(4, 0x50);
        memory.write8(5, 0x60);
        memory.write8(6, 0x70);
        memory.write8(7, 0x80);
        memory.write8(8, 0x90);

        assert_eq!(memory.read8(0), 0x10);
        assert_eq!(memory.read16(1), 0x3020);
        assert_eq!(memory.read32(1), 0x50403020);
        assert_eq!(memory.read64(1), 0x9080706050403020);
        assert_eq!(memory.read64(0), 0x8070605040302010);

        assert_eq!(memory.read16(2), 0x4030);
        assert_eq!(memory.read32(2), 0x60504030);

        memory.write64(9, 0x1122334455667788);
        assert_eq!(memory.read8(9), 0x88);
        assert_eq!(memory.read8(10), 0x77);
        assert_eq!(memory.read8(11), 0x66);
        assert_eq!(memory.read8(12), 0x55);
        assert_eq!(memory.read8(13), 0x44);
        assert_eq!(memory.read8(14), 0x33);
        assert_eq!(memory.read8(15), 0x22);
        assert_eq!(memory.read8(16), 0x11);

        assert_eq!(memory.read16(15), 0x1122);
        assert_eq!(memory.read32(11), 0x33445566);
    }
}
