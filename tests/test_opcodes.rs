extern crate rz80;

#[cfg(test)]
#[allow(unused_imports)]
mod test_opcodes {
    use std::cell::Cell;
    use rz80;
    use rz80::Register8::*;
    use rz80::Register16::*;
    use rz80::{RegT, CF, NF, VF, PF, XF, HF, YF, ZF, SF};

    struct TestBus {
        pub port: Cell<RegT>,
        pub val: Cell<RegT>,
    }
    impl TestBus {
        pub fn new() -> TestBus {
            TestBus {
                port: Cell::new(0),
                val: Cell::new(0),
            }
        }
    }
    impl rz80::Bus for TestBus {
        fn cpu_inp(&self, port: RegT) -> RegT {
            (port * 2) & 0xFF
        }
        fn cpu_outp(&self, port: RegT, val: RegT) {
            self.port.set(port);
            self.val.set(val);
        }
    }

    fn flags(cpu: &rz80::CPU, expected: rz80::RegT) -> bool {
        (cpu.reg.get8(F) & !(XF|YF)) == expected
    }

    #[test]
    fn test_ld_r_s() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let prog = [
            0x47,       // LD B,A
            0x4F,       // LD C,A
            0x57,       // LD D,A
            0x5F,       // LD E,A
            0x67,       // LD H,A
            0x6F,       // LD L,A
            0x7F,       // LD A,A

            0x48,       // LD C,B
            0x51,       // LD D,C
            0x5A,       // LD E,D
            0x63,       // LD H,E
            0x6C,       // LD L,H
            0x7D,       // LD A,L

        ];
        cpu.mem.write(0x0000, &prog);

        cpu.reg.set8(A, 0x12);
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x12, cpu.reg.get8(B));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x12, cpu.reg.get8(C));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x12, cpu.reg.get8(D));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x12, cpu.reg.get8(E));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x12, cpu.reg.get8(H));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x12, cpu.reg.get8(L));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x12, cpu.reg.get8(A));
        cpu.reg.set8(B, 0x13);
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x13, cpu.reg.get8(C));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x13, cpu.reg.get8(D));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x13, cpu.reg.get8(E));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x13, cpu.reg.get8(H));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x13, cpu.reg.get8(L));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x13, cpu.reg.get8(A));
    }

    #[test]
    fn test_ld_ihl() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let prog = [
            0x77,       // LD (HL),A
            0x46,       // LD B,(HL)
            0x4E,       // LD C,(HL)
            0x56,       // LD D,(HL)
            0x5E,       // LD E,(HL)
            0x66,       // LD H,(HL)
        ];
        cpu.mem.write(0x0100, &prog);

        cpu.reg.set8(A, 0x33);
        cpu.reg.set16(HL, 0x1000);
        cpu.reg.set_pc(0x0100);
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x33, cpu.mem.r8(0x1000));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x33, cpu.reg.get8(B));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x33, cpu.reg.get8(C));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x33, cpu.reg.get8(D));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x33, cpu.reg.get8(E));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x33, cpu.reg.get8(H));
    }

    #[test]
    fn test_ld_ihl_n() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let prog = [
            0x21, 0x00, 0x20,   // LD HL,0x2000
            0x36, 0x33,         // LD (HL),0x33
            0x21, 0x00, 0x10,   // LD HL,0x1000
            0x36, 0x65,         // LD (HL),0x65
        ];
        cpu.mem.write(0x0000, &prog);

        assert_eq!(10, cpu.step(bus)); assert_eq!(0x2000, cpu.reg.get16(HL));
        assert_eq!(10, cpu.step(bus)); assert_eq!(0x33, cpu.mem.r8(0x2000));
        assert_eq!(10, cpu.step(bus)); assert_eq!(0x1000, cpu.reg.get16(HL));
        assert_eq!(10, cpu.step(bus)); assert_eq!(0x65, cpu.mem.r8(0x1000));
    }

    #[test]
    fn test_ld_ixiy_n() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let prog = [
            0xDD, 0x21, 0x00, 0x20,     // LD IX,0x2000
            0xDD, 0x36, 0x02, 0x33,     // LD (IX+2),0x33
            0xDD, 0x36, 0xFE, 0x11,     // LD (IX-2),0x11
            0xFD, 0x21, 0x00, 0x10,     // LD IY,0x1000
            0xFD, 0x36, 0x01, 0x22,     // LD (IY+1),0x22
            0xFD, 0x36, 0xFF, 0x44,     // LD (IY-1),0x44
        ];
        cpu.mem.write(0x0000, &prog);

        assert_eq!(14, cpu.step(bus)); assert_eq!(0x2000, cpu.reg.get16(IX));
        assert_eq!(19, cpu.step(bus)); assert_eq!(0x33, cpu.mem.r8(0x2002));
        assert_eq!(19, cpu.step(bus)); assert_eq!(0x11, cpu.mem.r8(0x1FFE));
        assert_eq!(14, cpu.step(bus)); assert_eq!(0x1000, cpu.reg.get16(IY));
        assert_eq!(19, cpu.step(bus)); assert_eq!(0x22, cpu.mem.r8(0x1001));
        assert_eq!(19, cpu.step(bus)); assert_eq!(0x44, cpu.mem.r8(0x0FFF));
    }

    #[test]
    fn test_ld_ddixiy_nn() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let prog = [
            0x01, 0x34, 0x12,       // LD BC,0x1234
            0x11, 0x78, 0x56,       // LD DE,0x5678
            0x21, 0xBC, 0x9A,       // LD HL,0x9ABC
            0x31, 0x68, 0x13,       // LD SP,0x1368
            0xDD, 0x21, 0x21, 0x43, // LD IX,0x4321
            0xFD, 0x21, 0x65, 0x87, // LD IY,0x8765
        ];
        cpu.mem.write(0x0000, &prog);

        assert_eq!(10, cpu.step(bus)); assert_eq!(0x1234, cpu.reg.get16(BC));
        assert_eq!(10, cpu.step(bus)); assert_eq!(0x5678, cpu.reg.get16(DE));
        assert_eq!(10, cpu.step(bus)); assert_eq!(0x9ABC, cpu.reg.get16(HL));
        assert_eq!(10, cpu.step(bus)); assert_eq!(0x1368, cpu.reg.get16(SP));
        assert_eq!(14, cpu.step(bus)); assert_eq!(0x4321, cpu.reg.get16(IX));
        assert_eq!(14, cpu.step(bus)); assert_eq!(0x8765, cpu.reg.get16(IY));
    }

    #[test]
    fn test_ld_hlddixiy_inn() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let data = [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08
        ];
        cpu.mem.write(0x1000, &data);

        let prog = [
            0x2A, 0x00, 0x10,           // LD HL,(0x1000)
            0xED, 0x4B, 0x01, 0x10,     // LD BC,(0x1001)
            0xED, 0x5B, 0x02, 0x10,     // LD DE,(0x1002)
            0xED, 0x6B, 0x03, 0x10,     // LD HL,(0x1003) undocumented 'long' version
            0xED, 0x7B, 0x04, 0x10,     // LD SP,(0x1004)
            0xDD, 0x2A, 0x05, 0x10,     // LD IX,(0x1004)
            0xFD, 0x2A, 0x06, 0x10,     // LD IY,(0x1005)
        ];
        cpu.mem.write(0x0000, &prog);

        assert_eq!(16, cpu.step(bus)); assert_eq!(0x0201, cpu.reg.get16(HL));
        assert_eq!(20, cpu.step(bus)); assert_eq!(0x0302, cpu.reg.get16(BC));
        assert_eq!(20, cpu.step(bus)); assert_eq!(0x0403, cpu.reg.get16(DE));
        assert_eq!(20, cpu.step(bus)); assert_eq!(0x0504, cpu.reg.get16(HL));
        assert_eq!(20, cpu.step(bus)); assert_eq!(0x0605, cpu.reg.get16(SP));
        assert_eq!(20, cpu.step(bus)); assert_eq!(0x0706, cpu.reg.get16(IX));
        assert_eq!(20, cpu.step(bus)); assert_eq!(0x0807, cpu.reg.get16(IY));
    }

    #[test]
    fn test_ld_sp_hlixiy() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let prog = [
            0x21, 0x34, 0x12,           // LD HL,0x1234
            0xDD, 0x21, 0x78, 0x56,     // LD IX,0x5678
            0xFD, 0x21, 0xBC, 0x9A,     // LD IY,0x9ABC
            0xF9,                       // LD SP,HL
            0xDD, 0xF9,                 // LD SP,IX
            0xFD, 0xF9,                 // LD SP,IY
        ];
        cpu.mem.write(0x0000, &prog);

        assert_eq!(10, cpu.step(bus)); assert_eq!(0x1234, cpu.reg.get16(HL));
        assert_eq!(14, cpu.step(bus)); assert_eq!(0x5678, cpu.reg.get16(IX));
        assert_eq!(14, cpu.step(bus)); assert_eq!(0x9ABC, cpu.reg.get16(IY));
        assert_eq!(6, cpu.step(bus)); assert_eq!(0x1234, cpu.reg.get16(SP));
        assert_eq!(10, cpu.step(bus)); assert_eq!(0x5678, cpu.reg.get16(SP));
        assert_eq!(10, cpu.step(bus)); assert_eq!(0x9ABC, cpu.reg.get16(SP));
    }

    #[test]
    fn test_ld_r_ixiy() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let data = [
            1, 2, 3, 4, 5, 6, 7, 8
        ];
        cpu.mem.write(0x1000, &data);

        let  prog = [
            0xDD, 0x21, 0x03, 0x10,     // LD IX,0x1003
            0xDD, 0x7E, 0x00,           // LD A,(IX+0)
            0xDD, 0x46, 0x01,           // LD B,(IX+1)
            0xDD, 0x4E, 0x02,           // LD C,(IX+2)
            0xDD, 0x56, 0xFF,           // LD D,(IX-1)
            0xDD, 0x5E, 0xFE,           // LD E,(IX-2)
            0xDD, 0x66, 0x03,           // LD H,(IX+3)
            0xDD, 0x6E, 0xFD,           // LD L,(IX-3)

            0xFD, 0x21, 0x04, 0x10,     // LD IY,0x1003
            0xFD, 0x7E, 0x00,           // LD A,(IY+0)
            0xFD, 0x46, 0x01,           // LD B,(IY+1)
            0xFD, 0x4E, 0x02,           // LD C,(IY+2)
            0xFD, 0x56, 0xFF,           // LD D,(IY-1)
            0xFD, 0x5E, 0xFE,           // LD E,(IY-2)
            0xFD, 0x66, 0x03,           // LD H,(IY+3)
            0xFD, 0x6E, 0xFD,           // LD L,(IY-3)
        ];
        cpu.mem.write(0x0000, &prog);

        assert_eq!(14, cpu.step(bus)); assert_eq!(0x1003, cpu.reg.get16(IX));
        assert_eq!(19, cpu.step(bus)); assert_eq!(4, cpu.reg.get8(A));
        assert_eq!(19, cpu.step(bus)); assert_eq!(5, cpu.reg.get8(B));
        assert_eq!(19, cpu.step(bus)); assert_eq!(6, cpu.reg.get8(C));
        assert_eq!(19, cpu.step(bus)); assert_eq!(3, cpu.reg.get8(D));
        assert_eq!(19, cpu.step(bus)); assert_eq!(2, cpu.reg.get8(E));
        assert_eq!(19, cpu.step(bus)); assert_eq!(7, cpu.reg.get8(H));
        assert_eq!(19, cpu.step(bus)); assert_eq!(1, cpu.reg.get8(L));
        assert_eq!(14, cpu.step(bus)); assert_eq!(0x1004, cpu.reg.get16(IY));
        assert_eq!(19, cpu.step(bus)); assert_eq!(5, cpu.reg.get8(A));
        assert_eq!(19, cpu.step(bus)); assert_eq!(6, cpu.reg.get8(B));
        assert_eq!(19, cpu.step(bus)); assert_eq!(7, cpu.reg.get8(C));
        assert_eq!(19, cpu.step(bus)); assert_eq!(4, cpu.reg.get8(D));
        assert_eq!(19, cpu.step(bus)); assert_eq!(3, cpu.reg.get8(E));
        assert_eq!(19, cpu.step(bus)); assert_eq!(8, cpu.reg.get8(H));
        assert_eq!(19, cpu.step(bus)); assert_eq!(2, cpu.reg.get8(L));
    }

    #[test]
    fn test_ld_ixiy_r() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let prog = [
            0xDD, 0x21, 0x03, 0x10,     // LD IX,0x1003
            0x3E, 0x12,                 // LD A,0x12
            0xDD, 0x77, 0x00,           // LD (IX+0),A
            0x06, 0x13,                 // LD B,0x13
            0xDD, 0x70, 0x01,           // LD (IX+1),B
            0x0E, 0x14,                 // LD C,0x14
            0xDD, 0x71, 0x02,           // LD (IX+2),C
            0x16, 0x15,                 // LD D,0x15
            0xDD, 0x72, 0xFF,           // LD (IX-1),D
            0x1E, 0x16,                 // LD E,0x16
            0xDD, 0x73, 0xFE,           // LD (IX-2),E
            0x26, 0x17,                 // LD H,0x17
            0xDD, 0x74, 0x03,           // LD (IX+3),H
            0x2E, 0x18,                 // LD L,0x18
            0xDD, 0x75, 0xFD,           // LD (IX-3),L
            0xFD, 0x21, 0x03, 0x10,     // LD IY,0x1003
            0x3E, 0x12,                 // LD A,0x12
            0xFD, 0x77, 0x00,           // LD (IY+0),A
            0x06, 0x13,                 // LD B,0x13
            0xFD, 0x70, 0x01,           // LD (IY+1),B
            0x0E, 0x14,                 // LD C,0x14
            0xFD, 0x71, 0x02,           // LD (IY+2),C
            0x16, 0x15,                 // LD D,0x15
            0xFD, 0x72, 0xFF,           // LD (IY-1),D
            0x1E, 0x16,                 // LD E,0x16
            0xFD, 0x73, 0xFE,           // LD (IY-2),E
            0x26, 0x17,                 // LD H,0x17
            0xFD, 0x74, 0x03,           // LD (IY+3),H
            0x2E, 0x18,                 // LD L,0x18
            0xFD, 0x75, 0xFD,           // LD (IY-3),L
        ];
        cpu.mem.write(0x0000, &prog);

        assert_eq!(14, cpu.step(bus)); assert_eq!(0x1003, cpu.reg.get16(IX));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x12, cpu.reg.get8(A));
        assert_eq!(19, cpu.step(bus)); assert_eq!(0x12, cpu.mem.r8(0x1003));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x13, cpu.reg.get8(B));
        assert_eq!(19, cpu.step(bus)); assert_eq!(0x13, cpu.mem.r8(0x1004));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x14, cpu.reg.get8(C));
        assert_eq!(19, cpu.step(bus)); assert_eq!(0x14, cpu.mem.r8(0x1005));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x15, cpu.reg.get8(D));
        assert_eq!(19, cpu.step(bus)); assert_eq!(0x15, cpu.mem.r8(0x1002));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x16, cpu.reg.get8(E));
        assert_eq!(19, cpu.step(bus)); assert_eq!(0x16, cpu.mem.r8(0x1001));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x17, cpu.reg.get8(H));
        assert_eq!(19, cpu.step(bus)); assert_eq!(0x17, cpu.mem.r8(0x1006));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x18, cpu.reg.get8(L));
        assert_eq!(19, cpu.step(bus)); assert_eq!(0x18, cpu.mem.r8(0x1000));
        assert_eq!(14, cpu.step(bus)); assert_eq!(0x1003, cpu.reg.get16(IY));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x12, cpu.reg.get8(A));
        assert_eq!(19, cpu.step(bus)); assert_eq!(0x12, cpu.mem.r8(0x1003));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x13, cpu.reg.get8(B));
        assert_eq!(19, cpu.step(bus)); assert_eq!(0x13, cpu.mem.r8(0x1004));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x14, cpu.reg.get8(C));
        assert_eq!(19, cpu.step(bus)); assert_eq!(0x14, cpu.mem.r8(0x1005));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x15, cpu.reg.get8(D));
        assert_eq!(19, cpu.step(bus)); assert_eq!(0x15, cpu.mem.r8(0x1002));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x16, cpu.reg.get8(E));
        assert_eq!(19, cpu.step(bus)); assert_eq!(0x16, cpu.mem.r8(0x1001));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x17, cpu.reg.get8(H));
        assert_eq!(19, cpu.step(bus)); assert_eq!(0x17, cpu.mem.r8(0x1006));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x18, cpu.reg.get8(L));
        assert_eq!(19, cpu.step(bus)); assert_eq!(0x18, cpu.mem.r8(0x1000));
    }

    #[test]
    fn test_push_pop() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let prog = [
            0x01, 0x34, 0x12,       // LD BC,0x1234
            0x11, 0x78, 0x56,       // LD DE,0x5678
            0x21, 0xBC, 0x9A,       // LD HL,0x9ABC
            0x3E, 0xEF,             // LD A,0xEF
            0xDD, 0x21, 0x45, 0x23, // LD IX,0x2345
            0xFD, 0x21, 0x89, 0x67, // LD IY,0x6789
            0x31, 0x00, 0x01,       // LD SP,0x0100
            0xF5,                   // PUSH AF
            0xC5,                   // PUSH BC
            0xD5,                   // PUSH DE
            0xE5,                   // PUSH HL
            0xDD, 0xE5,             // PUSH IX
            0xFD, 0xE5,             // PUSH IY
            0xF1,                   // POP AF
            0xC1,                   // POP BC
            0xD1,                   // POP DE
            0xE1,                   // POP HL
            0xDD, 0xE1,             // POP IX
            0xFD, 0xE1,             // POP IY
        ];
        cpu.mem.write(0x0000, &prog);

        assert_eq!(10, cpu.step(bus)); assert_eq!(0x1234, cpu.reg.get16(BC));
        assert_eq!(10, cpu.step(bus)); assert_eq!(0x5678, cpu.reg.get16(DE));
        assert_eq!(10, cpu.step(bus)); assert_eq!(0x9ABC, cpu.reg.get16(HL));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0xEF00, cpu.reg.get16(AF));
        assert_eq!(14, cpu.step(bus)); assert_eq!(0x2345, cpu.reg.get16(IX));
        assert_eq!(14, cpu.step(bus)); assert_eq!(0x6789, cpu.reg.get16(IY));
        assert_eq!(10, cpu.step(bus)); assert_eq!(0x0100, cpu.reg.get16(SP));
        assert_eq!(11, cpu.step(bus)); assert_eq!(0xEF00, cpu.mem.r16(0x00FE)); assert_eq!(0x00FE, cpu.reg.get16(SP));
        assert_eq!(11, cpu.step(bus)); assert_eq!(0x1234, cpu.mem.r16(0x00FC)); assert_eq!(0x00FC, cpu.reg.get16(SP));
        assert_eq!(11, cpu.step(bus)); assert_eq!(0x5678, cpu.mem.r16(0x00FA)); assert_eq!(0x00FA, cpu.reg.get16(SP));
        assert_eq!(11, cpu.step(bus)); assert_eq!(0x9ABC, cpu.mem.r16(0x00F8)); assert_eq!(0x00F8, cpu.reg.get16(SP));
        assert_eq!(15, cpu.step(bus)); assert_eq!(0x2345, cpu.mem.r16(0x00F6)); assert_eq!(0x00F6, cpu.reg.get16(SP));
        assert_eq!(15, cpu.step(bus)); assert_eq!(0x6789, cpu.mem.r16(0x00F4)); assert_eq!(0x00F4, cpu.reg.get16(SP));
        assert_eq!(10, cpu.step(bus)); assert_eq!(0x6789, cpu.reg.get16(AF)); assert_eq!(0x00F6, cpu.reg.get16(SP));
        assert_eq!(10, cpu.step(bus)); assert_eq!(0x2345, cpu.reg.get16(BC)); assert_eq!(0x00F8, cpu.reg.get16(SP));
        assert_eq!(10, cpu.step(bus)); assert_eq!(0x9ABC, cpu.reg.get16(DE)); assert_eq!(0x00FA, cpu.reg.get16(SP));
        assert_eq!(10, cpu.step(bus)); assert_eq!(0x5678, cpu.reg.get16(HL)); assert_eq!(0x00FC, cpu.reg.get16(SP));
        assert_eq!(14, cpu.step(bus)); assert_eq!(0x1234, cpu.reg.get16(IX)); assert_eq!(0x00FE, cpu.reg.get16(SP));
        assert_eq!(14, cpu.step(bus)); assert_eq!(0xEF00, cpu.reg.get16(IY)); assert_eq!(0x0100, cpu.reg.get16(SP));
    }

    #[test]
    fn test_add_r() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let prog = [
            0x3E, 0x0F,     // LD A,0x0F
            0x87,           // ADD A,A
            0x06, 0xE0,     // LD B,0xE0
            0x80,           // ADD A,B
            0x3E, 0x81,     // LD A,0x81
            0x0E, 0x80,     // LD C,0x80
            0x81,           // ADD A,C
            0x16, 0xFF,     // LD D,0xFF
            0x82,           // ADD A,D
            0x1E, 0x40,     // LD E,0x40
            0x83,           // ADD A,E
            0x26, 0x80,     // LD H,0x80
            0x84,           // ADD A,H
            0x2E, 0x33,     // LD L,0x33
            0x85,           // ADD A,L
            0xC6, 0x44,     // ADD A,0x44
        ];
        cpu.mem.write(0x0000, &prog);

        assert_eq!(7, cpu.step(bus)); assert_eq!(0x0F, cpu.reg.get8(A)); assert!(flags(&cpu, 0));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x1E, cpu.reg.get8(A)); assert!(flags(&cpu, HF));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0xE0, cpu.reg.get8(B));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0xFE, cpu.reg.get8(A)); assert!(flags(&cpu, SF));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x81, cpu.reg.get8(A));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x80, cpu.reg.get8(C));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x01, cpu.reg.get8(A)); assert!(flags(&cpu, VF|CF));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0xFF, cpu.reg.get8(D));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x00, cpu.reg.get8(A)); assert!(flags(&cpu, ZF|HF|CF));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x40, cpu.reg.get8(E));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x40, cpu.reg.get8(A)); assert!(flags(&cpu, 0));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x80, cpu.reg.get8(H));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0xC0, cpu.reg.get8(A)); assert!(flags(&cpu, SF));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x33, cpu.reg.get8(L));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0xF3, cpu.reg.get8(A)); assert!(flags(&cpu, SF));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x37, cpu.reg.get8(A)); assert!(flags(&cpu, CF));
    }

    #[test]
    fn test_add_ihlixiy() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let data = [ 0x41, 0x61, 0x81 ];
        cpu.mem.write(0x1000, &data);

        let prog = [
            0x21, 0x00, 0x10,       // LD HL,0x1000
            0xDD, 0x21, 0x00, 0x10, // LD IX,0x1000
            0xFD, 0x21, 0x03, 0x10, // LD IY,0x1003
            0x3E, 0x00,             // LD A,0x00
            0x86,                   // ADD A,(HL)
            0xDD, 0x86, 0x01,       // ADD A,(IX+1)
            0xFD, 0x86, 0xFF,       // ADD A,(IY-1)
        ];
        cpu.mem.write(0x0000, &prog);

        assert_eq!(10, cpu.step(bus)); assert_eq!(0x1000, cpu.reg.get16(HL));
        assert_eq!(14, cpu.step(bus)); assert_eq!(0x1000, cpu.reg.get16(IX));
        assert_eq!(14, cpu.step(bus)); assert_eq!(0x1003, cpu.reg.get16(IY));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x00, cpu.reg.get8(A));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x41, cpu.reg.get8(A)); assert!(flags(&cpu, 0));
        assert_eq!(19, cpu.step(bus)); assert_eq!(0xA2, cpu.reg.get8(A)); assert!(flags(&cpu, SF|VF));
        assert_eq!(19, cpu.step(bus)); assert_eq!(0x23, cpu.reg.get8(A)); assert!(flags(&cpu, VF|CF));
    }

    #[test]
    fn test_adc_r() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let prog = [
            0x3E, 0x00,         // LD A,0x00
            0x06, 0x41,         // LD B,0x41
            0x0E, 0x61,         // LD C,0x61
            0x16, 0x81,         // LD D,0x81
            0x1E, 0x41,         // LD E,0x41
            0x26, 0x61,         // LD H,0x61
            0x2E, 0x81,         // LD L,0x81
            0x8F,               // ADC A,A
            0x88,               // ADC A,B
            0x89,               // ADC A,C
            0x8A,               // ADC A,D
            0x8B,               // ADC A,E
            0x8C,               // ADC A,H
            0x8D,               // ADC A,L
            0xCE, 0x01,         // ADC A,0x01
        ];
        cpu.mem.write(0x0000, &prog);

        assert_eq!(7, cpu.step(bus)); assert_eq!(0x00, cpu.reg.get8(A));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x41, cpu.reg.get8(B));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x61, cpu.reg.get8(C));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x81, cpu.reg.get8(D));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x41, cpu.reg.get8(E));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x61, cpu.reg.get8(H));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x81, cpu.reg.get8(L));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x00, cpu.reg.get8(A)); assert!(flags(&cpu, ZF));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x41, cpu.reg.get8(A)); assert!(flags(&cpu, 0));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0xA2, cpu.reg.get8(A)); assert!(flags(&cpu, SF|VF));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x23, cpu.reg.get8(A)); assert!(flags(&cpu, VF|CF));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x65, cpu.reg.get8(A)); assert!(flags(&cpu, 0));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0xC6, cpu.reg.get8(A)); assert!(flags(&cpu, SF|VF));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x47, cpu.reg.get8(A)); assert!(flags(&cpu, VF|CF));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x49, cpu.reg.get8(A)); assert!(flags(&cpu, 0));
    }

    #[test]
    fn test_adc_ihlixiy() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let data = [ 0x41, 0x61, 0x81, 0x2 ];
        cpu.mem.write(0x1000, &data);
        let prog = [
            0x21, 0x00, 0x10,       // LD HL,0x1000
            0xDD, 0x21, 0x00, 0x10, // LD IX,0x1000
            0xFD, 0x21, 0x03, 0x10, // LD IY,0x1003
            0x3E, 0x00,             // LD A,0x00
            0x86,                   // ADD A,(HL)
            0xDD, 0x8E, 0x01,       // ADC A,(IX+1)
            0xFD, 0x8E, 0xFF,       // ADC A,(IY-1)
            0xDD, 0x8E, 0x03,       // ADC A,(IX+3)
        ];
        cpu.mem.write(0x0000, &prog);

        assert_eq!(10, cpu.step(bus)); assert_eq!(0x1000, cpu.reg.get16(HL));
        assert_eq!(14, cpu.step(bus)); assert_eq!(0x1000, cpu.reg.get16(IX));
        assert_eq!(14, cpu.step(bus)); assert_eq!(0x1003, cpu.reg.get16(IY));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x00, cpu.reg.get8(A));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x41, cpu.reg.get8(A)); assert!(flags(&cpu, 0));
        assert_eq!(19, cpu.step(bus)); assert_eq!(0xA2, cpu.reg.get8(A)); assert!(flags(&cpu, SF|VF));
        assert_eq!(19, cpu.step(bus)); assert_eq!(0x23, cpu.reg.get8(A)); assert!(flags(&cpu, VF|CF));
        assert_eq!(19, cpu.step(bus)); assert_eq!(0x26, cpu.reg.get8(A)); assert!(flags(&cpu, 0));
    }

    #[test]
    fn test_sub_r() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let prog = [
            0x3E, 0x04,     // LD A,0x04
            0x06, 0x01,     // LD B,0x01
            0x0E, 0xF8,     // LD C,0xF8
            0x16, 0x0F,     // LD D,0x0F
            0x1E, 0x79,     // LD E,0x79
            0x26, 0xC0,     // LD H,0xC0
            0x2E, 0xBF,     // LD L,0xBF
            0x97,           // SUB A,A
            0x90,           // SUB A,B
            0x91,           // SUB A,C
            0x92,           // SUB A,D
            0x93,           // SUB A,E
            0x94,           // SUB A,H
            0x95,           // SUB A,L
            0xD6, 0x01,     // SUB A,0x01
            0xD6, 0xFE,     // SUB A,0xFE
        ];
        cpu.mem.write(0x0000, &prog);

        assert_eq!(7, cpu.step(bus)); assert_eq!(0x04, cpu.reg.get8(A));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x01, cpu.reg.get8(B));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0xF8, cpu.reg.get8(C));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x0F, cpu.reg.get8(D));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x79, cpu.reg.get8(E));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0xC0, cpu.reg.get8(H));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0xBF, cpu.reg.get8(L));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x00, cpu.reg.get8(A)); assert!(flags(&cpu, ZF|NF));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0xFF, cpu.reg.get8(A)); assert!(flags(&cpu, SF|HF|NF|CF));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x07, cpu.reg.get8(A)); assert!(flags(&cpu, NF));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0xF8, cpu.reg.get8(A)); assert!(flags(&cpu, SF|HF|NF|CF));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x7F, cpu.reg.get8(A)); assert!(flags(&cpu, HF|VF|NF));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0xBF, cpu.reg.get8(A)); assert!(flags(&cpu, SF|VF|NF|CF));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x00, cpu.reg.get8(A)); assert!(flags(&cpu, ZF|NF));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0xFF, cpu.reg.get8(A)); assert!(flags(&cpu, SF|HF|NF|CF));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x01, cpu.reg.get8(A)); assert!(flags(&cpu, NF));
    }

    #[test]
    fn test_cp_r() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let prog = [
            0x3E, 0x04,     // LD A,0x04
            0x06, 0x05,     // LD B,0x05
            0x0E, 0x03,     // LD C,0x03
            0x16, 0xff,     // LD D,0xff
            0x1E, 0xaa,     // LD E,0xaa
            0x26, 0x80,     // LD H,0x80
            0x2E, 0x7f,     // LD L,0x7f
            0xBF,           // CP A
            0xB8,           // CP B
            0xB9,           // CP C
            0xBA,           // CP D
            0xBB,           // CP E
            0xBC,           // CP H
            0xBD,           // CP L
            0xFE, 0x04,     // CP 0x04
        ];
        cpu.mem.write(0x0000, &prog);

        assert_eq!(7, cpu.step(bus)); assert_eq!(0x04, cpu.reg.get8(A));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x05, cpu.reg.get8(B));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x03, cpu.reg.get8(C));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0xff, cpu.reg.get8(D));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0xaa, cpu.reg.get8(E));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x80, cpu.reg.get8(H));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x7f, cpu.reg.get8(L));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x04, cpu.reg.get8(A)); assert!(flags(&cpu, ZF|NF));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x04, cpu.reg.get8(A)); assert!(flags(&cpu, SF|HF|NF|CF));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x04, cpu.reg.get8(A)); assert!(flags(&cpu, NF));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x04, cpu.reg.get8(A)); assert!(flags(&cpu, HF|NF|CF));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x04, cpu.reg.get8(A)); assert!(flags(&cpu, HF|NF|CF));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x04, cpu.reg.get8(A)); assert!(flags(&cpu, SF|VF|NF|CF));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x04, cpu.reg.get8(A)); assert!(flags(&cpu, SF|HF|NF|CF));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x04, cpu.reg.get8(A)); assert!(flags(&cpu, ZF|NF));
    }

    #[test]
    fn test_sub_ihlixiy() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let data = [ 0x41, 0x61, 0x81 ];
        cpu.mem.write(0x1000, &data);
        let prog = [
            0x21, 0x00, 0x10,       // LD HL,0x1000
            0xDD, 0x21, 0x00, 0x10, // LD IX,0x1000
            0xFD, 0x21, 0x03, 0x10, // LD IY,0x1003
            0x3E, 0x00,             // LD A,0x00
            0x96,                   // SUB A,(HL)
            0xDD, 0x96, 0x01,       // SUB A,(IX+1)
            0xFD, 0x96, 0xFE,       // SUB A,(IY-2)
        ];
        cpu.mem.write(0x0000, &prog);

        assert_eq!(10, cpu.step(bus)); assert_eq!(0x1000, cpu.reg.get16(HL));
        assert_eq!(14, cpu.step(bus)); assert_eq!(0x1000, cpu.reg.get16(IX));
        assert_eq!(14, cpu.step(bus)); assert_eq!(0x1003, cpu.reg.get16(IY));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x00, cpu.reg.get8(A));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0xBF, cpu.reg.get8(A)); assert!(flags(&cpu, SF|HF|NF|CF));
        assert_eq!(19, cpu.step(bus)); assert_eq!(0x5E, cpu.reg.get8(A)); assert!(flags(&cpu, VF|NF));
        assert_eq!(19, cpu.step(bus)); assert_eq!(0xFD, cpu.reg.get8(A)); assert!(flags(&cpu, SF|NF|CF));
    }

    #[test]
    fn test_cp_ihlixiy() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let data = [ 0x41, 0x61, 0x22 ];
        cpu.mem.write(0x1000, &data);
        let prog = [
            0x21, 0x00, 0x10,       // LD HL,0x1000
            0xDD, 0x21, 0x00, 0x10, // LD IX,0x1000
            0xFD, 0x21, 0x03, 0x10, // LD IY,0x1003
            0x3E, 0x41,             // LD A,0x41
            0xBE,                   // CP (HL)
            0xDD, 0xBE, 0x01,       // CP (IX+1)
            0xFD, 0xBE, 0xFF,       // CP (IY-1)
        ];
        cpu.mem.write(0x0000, &prog);

        assert_eq!(10, cpu.step(bus)); assert_eq!(0x1000, cpu.reg.get16(HL));
        assert_eq!(14, cpu.step(bus)); assert_eq!(0x1000, cpu.reg.get16(IX));
        assert_eq!(14, cpu.step(bus)); assert_eq!(0x1003, cpu.reg.get16(IY));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x41, cpu.reg.get8(A));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x41, cpu.reg.get8(A)); assert!(flags(&cpu, ZF|NF));
        assert_eq!(19, cpu.step(bus)); assert_eq!(0x41, cpu.reg.get8(A)); assert!(flags(&cpu, SF|NF|CF));
        assert_eq!(19, cpu.step(bus)); assert_eq!(0x41, cpu.reg.get8(A)); assert!(flags(&cpu, HF|NF));
    }

    #[test]
    fn test_sbc_r() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let prog = [
            0x3E, 0x04,     // LD A,0x04
            0x06, 0x01,     // LD B,0x01
            0x0E, 0xF8,     // LD C,0xF8
            0x16, 0x0F,     // LD D,0x0F
            0x1E, 0x79,     // LD E,0x79
            0x26, 0xC0,     // LD H,0xC0
            0x2E, 0xBF,     // LD L,0xBF
            0x97,           // SUB A,A
            0x98,           // SBC A,B
            0x99,           // SBC A,C
            0x9A,           // SBC A,D
            0x9B,           // SBC A,E
            0x9C,           // SBC A,H
            0x9D,           // SBC A,L
            0xDE, 0x01,     // SBC A,0x01
            0xDE, 0xFE,     // SBC A,0xFE
        ];
        cpu.mem.write(0x0000, &prog);

        for _ in 0..7 {
            cpu.step(bus);
        }
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x00, cpu.reg.get8(A)); assert!(flags(&cpu, ZF|NF));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0xFF, cpu.reg.get8(A)); assert!(flags(&cpu, SF|HF|NF|CF));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x06, cpu.reg.get8(A)); assert!(flags(&cpu, NF));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0xF7, cpu.reg.get8(A)); assert!(flags(&cpu, SF|HF|NF|CF));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x7D, cpu.reg.get8(A)); assert!(flags(&cpu, HF|VF|NF));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0xBD, cpu.reg.get8(A)); assert!(flags(&cpu, SF|VF|NF|CF));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0xFD, cpu.reg.get8(A)); assert!(flags(&cpu, SF|HF|NF|CF));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0xFB, cpu.reg.get8(A)); assert!(flags(&cpu, SF|NF));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0xFD, cpu.reg.get8(A)); assert!(flags(&cpu, SF|HF|NF|CF));
    }

    #[test]
    fn test_sbc_ihlixiy() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let data = [ 0x41, 0x61, 0x81 ];
        cpu.mem.write(0x1000, &data);
        let prog = [
            0x21, 0x00, 0x10,       // LD HL,0x1000
            0xDD, 0x21, 0x00, 0x10, // LD IX,0x1000
            0xFD, 0x21, 0x03, 0x10, // LD IY,0x1003
            0x3E, 0x00,             // LD A,0x00
            0x9E,                   // SBC A,(HL)
            0xDD, 0x9E, 0x01,       // SBC A,(IX+1)
            0xFD, 0x9E, 0xFE,       // SBC A,(IY-2)
        ];
        cpu.mem.write(0x0000, &prog);

        assert_eq!(10, cpu.step(bus)); assert_eq!(0x1000, cpu.reg.get16(HL));
        assert_eq!(14, cpu.step(bus)); assert_eq!(0x1000, cpu.reg.get16(IX));
        assert_eq!(14, cpu.step(bus)); assert_eq!(0x1003, cpu.reg.get16(IY));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x00, cpu.reg.get8(A));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0xBF, cpu.reg.get8(A)); assert!(flags(&cpu, SF|HF|NF|CF));
        assert_eq!(19, cpu.step(bus)); assert_eq!(0x5D, cpu.reg.get8(A)); assert!(flags(&cpu, VF|NF));
        assert_eq!(19, cpu.step(bus)); assert_eq!(0xFC, cpu.reg.get8(A)); assert!(flags(&cpu, SF|NF|CF));
    }

    #[test]
    fn test_or_r() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let prog = [
            0x97,           // SUB A
            0x06, 0x01,     // LD B,0x01
            0x0E, 0x02,     // LD C,0x02
            0x16, 0x04,     // LD D,0x04
            0x1E, 0x08,     // LD E,0x08
            0x26, 0x10,     // LD H,0x10
            0x2E, 0x20,     // LD L,0x20
            0xB7,           // OR A
            0xB0,           // OR B
            0xB1,           // OR C
            0xB2,           // OR D
            0xB3,           // OR E
            0xB4,           // OR H
            0xB5,           // OR L
            0xF6, 0x40,     // OR 0x40
            0xF6, 0x80,     // OR 0x80
        ];
        cpu.mem.write(0x0000, &prog);

        for _ in 0..7 {
            cpu.step(bus);
        }
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x00, cpu.reg.get8(A)); assert!(flags(&cpu, ZF|PF));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x01, cpu.reg.get8(A)); assert!(flags(&cpu, 0));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x03, cpu.reg.get8(A)); assert!(flags(&cpu, PF));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x07, cpu.reg.get8(A)); assert!(flags(&cpu, 0));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x0F, cpu.reg.get8(A)); assert!(flags(&cpu, PF));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x1F, cpu.reg.get8(A)); assert!(flags(&cpu, 0));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x3F, cpu.reg.get8(A)); assert!(flags(&cpu, PF));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x7F, cpu.reg.get8(A)); assert!(flags(&cpu, 0));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0xFF, cpu.reg.get8(A)); assert!(flags(&cpu, SF|PF));
    }

    #[test]
    fn test_xor_r() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let prog = [
            0x97,           // SUB A
            0x06, 0x01,     // LD B,0x01
            0x0E, 0x03,     // LD C,0x03
            0x16, 0x07,     // LD D,0x07
            0x1E, 0x0F,     // LD E,0x0F
            0x26, 0x1F,     // LD H,0x1F
            0x2E, 0x3F,     // LD L,0x3F
            0xAF,           // XOR A
            0xA8,           // XOR B
            0xA9,           // XOR C
            0xAA,           // XOR D
            0xAB,           // XOR E
            0xAC,           // XOR H
            0xAD,           // XOR L
            0xEE, 0x7F,     // XOR 0x7F
            0xEE, 0xFF,     // XOR 0xFF
        ];
        cpu.mem.write(0x0000, &prog);

        for _ in 0..7 {
            cpu.step(bus);
        }
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x00, cpu.reg.get8(A)); assert!(flags(&cpu, ZF|PF));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x01, cpu.reg.get8(A)); assert!(flags(&cpu, 0));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x02, cpu.reg.get8(A)); assert!(flags(&cpu, 0));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x05, cpu.reg.get8(A)); assert!(flags(&cpu, PF));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x0A, cpu.reg.get8(A)); assert!(flags(&cpu, PF));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x15, cpu.reg.get8(A)); assert!(flags(&cpu, 0));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x2A, cpu.reg.get8(A)); assert!(flags(&cpu, 0));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x55, cpu.reg.get8(A)); assert!(flags(&cpu, PF));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0xAA, cpu.reg.get8(A)); assert!(flags(&cpu, SF|PF));
    }

    #[test]
    fn test_or_xor_ihlixiy() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let data = [ 0x41, 0x62, 0x84 ];
        cpu.mem.write(0x1000, &data);
        let prog = [
            0x21, 0x00, 0x10,           // LD HL,0x1000
            0xDD, 0x21, 0x00, 0x10,     // LD IX,0x1000
            0xFD, 0x21, 0x03, 0x10,     // LD IY,0x1003
            0xB6,                       // OR (HL)
            0xDD, 0xB6, 0x01,           // OR (IX+1)
            0xFD, 0xB6, 0xFF,           // OR (IY-1)
            0xAE,                       // XOR (HL)
            0xDD, 0xAE, 0x01,           // XOR (IX+1)
            0xFD, 0xAE, 0xFF,           // XOR (IY-1)
        ];
        cpu.mem.write(0x0000, &prog);

        // skip loads
        for _ in 0..3 {
            cpu.step(bus);
        }
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x41, cpu.reg.get8(A)); assert!(flags(&cpu, PF));
        assert_eq!(19, cpu.step(bus)); assert_eq!(0x63, cpu.reg.get8(A)); assert!(flags(&cpu, PF));
        assert_eq!(19, cpu.step(bus)); assert_eq!(0xE7, cpu.reg.get8(A)); assert!(flags(&cpu, SF|PF));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0xA6, cpu.reg.get8(A)); assert!(flags(&cpu, SF|PF));
        assert_eq!(19, cpu.step(bus)); assert_eq!(0xC4, cpu.reg.get8(A)); assert!(flags(&cpu, SF));
        assert_eq!(19, cpu.step(bus)); assert_eq!(0x40, cpu.reg.get8(A)); assert!(flags(&cpu, 0));
    }

    #[test]
    fn test_and_r() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let prog = [
            0x3E, 0xFF,             // LD A,0xFF
            0x06, 0x01,             // LD B,0x01
            0x0E, 0x03,             // LD C,0x02
            0x16, 0x04,             // LD D,0x04
            0x1E, 0x08,             // LD E,0x08
            0x26, 0x10,             // LD H,0x10
            0x2E, 0x20,             // LD L,0x20
            0xA0,                   // AND B
            0xF6, 0xFF,             // OR 0xFF
            0xA1,                   // AND C
            0xF6, 0xFF,             // OR 0xFF
            0xA2,                   // AND D
            0xF6, 0xFF,             // OR 0xFF
            0xA3,                   // AND E
            0xF6, 0xFF,             // OR 0xFF
            0xA4,                   // AND H
            0xF6, 0xFF,             // OR 0xFF
            0xA5,                   // AND L
            0xF6, 0xFF,             // OR 0xFF
            0xE6, 0x40,             // AND 0x40
            0xF6, 0xFF,             // OR 0xFF
            0xE6, 0xAA,             // AND 0xAA
        ];
        cpu.mem.write(0x0000, &prog);

        for _ in 0..7 {
            cpu.step(bus);
        }
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x01, cpu.reg.get8(A)); assert!(flags(&cpu, HF));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0xFF, cpu.reg.get8(A)); assert!(flags(&cpu, SF|PF));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x03, cpu.reg.get8(A)); assert!(flags(&cpu, HF|PF));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0xFF, cpu.reg.get8(A)); assert!(flags(&cpu, SF|PF));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x04, cpu.reg.get8(A)); assert!(flags(&cpu, HF));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0xFF, cpu.reg.get8(A)); assert!(flags(&cpu, SF|PF));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x08, cpu.reg.get8(A)); assert!(flags(&cpu, HF));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0xFF, cpu.reg.get8(A)); assert!(flags(&cpu, SF|PF));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x10, cpu.reg.get8(A)); assert!(flags(&cpu, HF));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0xFF, cpu.reg.get8(A)); assert!(flags(&cpu, SF|PF));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x20, cpu.reg.get8(A)); assert!(flags(&cpu, HF));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0xFF, cpu.reg.get8(A)); assert!(flags(&cpu, SF|PF));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x40, cpu.reg.get8(A)); assert!(flags(&cpu, HF));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0xFF, cpu.reg.get8(A)); assert!(flags(&cpu, SF|PF));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0xAA, cpu.reg.get8(A)); assert!(flags(&cpu, SF|HF|PF));
    }

    #[test]
    fn test_and_ihlixiy() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let data = [ 0xFE, 0xAA, 0x99 ];
        cpu.mem.write(0x1000, &data);
        let prog = [
            0x21, 0x00, 0x10,           // LD HL,0x1000
            0xDD, 0x21, 0x00, 0x10,     // LD IX,0x1000
            0xFD, 0x21, 0x03, 0x10,     // LD IY,0x1003
            0x3E, 0xFF,                 // LD A,0xFF
            0xA6,                       // AND (HL)
            0xDD, 0xA6, 0x01,           // AND (IX+1)
            0xFD, 0xA6, 0xFF,           // AND (IX-1)
        ];
        cpu.mem.write(0x0000, &prog);

        // skip loads
        for _ in 0..4 {
            cpu.step(bus);
        }
        assert_eq!(7, cpu.step(bus)); assert_eq!(0xFE, cpu.reg.get8(A)); assert!(flags(&cpu, SF|HF));
        assert_eq!(19, cpu.step(bus)); assert_eq!(0xAA, cpu.reg.get8(A)); assert!(flags(&cpu, SF|HF|PF));
        assert_eq!(19, cpu.step(bus)); assert_eq!(0x88, cpu.reg.get8(A)); assert!(flags(&cpu, SF|HF|PF));
    }

    #[test]
    fn test_inc_dec_r() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let prog = [
            0x3e, 0x00,         // LD A,0x00
            0x06, 0xFF,         // LD B,0xFF
            0x0e, 0x0F,         // LD C,0x0F
            0x16, 0x0E,         // LD D,0x0E
            0x1E, 0x7F,         // LD E,0x7F
            0x26, 0x3E,         // LD H,0x3E
            0x2E, 0x23,         // LD L,0x23
            0x3C,               // INC A
            0x3D,               // DEC A
            0x04,               // INC B
            0x05,               // DEC B
            0x0C,               // INC C
            0x0D,               // DEC C
            0x14,               // INC D
            0x15,               // DEC D
            0xFE, 0x01,         // CP 0x01  // set carry flag (should be preserved)
            0x1C,               // INC E
            0x1D,               // DEC E
            0x24,               // INC H
            0x25,               // DEC H
            0x2C,               // INC L
            0x2D,               // DEC L
        ];
        cpu.mem.write(0x0000, &prog);

        for _ in 0..7 {
            cpu.step(bus);
        }
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x01, cpu.reg.get8(A)); assert!(flags(&cpu, 0));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x00, cpu.reg.get8(A)); assert!(flags(&cpu, ZF|NF));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x00, cpu.reg.get8(B)); assert!(flags(&cpu, ZF|HF));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0xFF, cpu.reg.get8(B)); assert!(flags(&cpu, SF|HF|NF));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x10, cpu.reg.get8(C)); assert!(flags(&cpu, HF));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x0F, cpu.reg.get8(C)); assert!(flags(&cpu, HF|NF));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x0F, cpu.reg.get8(D)); assert!(flags(&cpu, 0));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x0E, cpu.reg.get8(D)); assert!(flags(&cpu, NF));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x00, cpu.reg.get8(A)); assert!(flags(&cpu, SF|HF|NF|CF));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x80, cpu.reg.get8(E)); assert!(flags(&cpu, SF|HF|VF|CF));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x7F, cpu.reg.get8(E)); assert!(flags(&cpu, HF|VF|NF|CF));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x3F, cpu.reg.get8(H)); assert!(flags(&cpu, CF));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x3E, cpu.reg.get8(H)); assert!(flags(&cpu, NF|CF));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x24, cpu.reg.get8(L)); assert!(flags(&cpu, CF));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x23, cpu.reg.get8(L)); assert!(flags(&cpu, NF|CF));
    }

    #[test]
    fn test_inc_dec_ihlixiy() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let data = [ 0x00, 0x3F, 0x7F ];
        cpu.mem.write(0x1000, &data);
        let prog = [
            0x21, 0x00, 0x10,           // LD HL,0x1000
            0xDD, 0x21, 0x00, 0x10,     // LD IX,0x1000
            0xFD, 0x21, 0x03, 0x10,     // LD IY,0x1003
            0x35,                       // DEC (HL)
            0x34,                       // INC (HL)
            0xDD, 0x34, 0x01,           // INC (IX+1)
            0xDD, 0x35, 0x01,           // DEC (IX+1)
            0xFD, 0x34, 0xFF,           // INC (IY-1)
            0xFD, 0x35, 0xFF,           // DEC (IY-1)
        ];
        cpu.mem.write(0x0000, &prog);

        // skip loads
        for _ in 0..3 {
            cpu.step(bus);
        }
        assert_eq!(11, cpu.step(bus)); assert_eq!(0xFF, cpu.mem.r8(0x1000)); assert!(flags(&cpu, SF|HF|NF));
        assert_eq!(11, cpu.step(bus)); assert_eq!(0x00, cpu.mem.r8(0x1000)); assert!(flags(&cpu, ZF|HF));
        assert_eq!(23, cpu.step(bus)); assert_eq!(0x40, cpu.mem.r8(0x1001)); assert!(flags(&cpu, HF));
        assert_eq!(23, cpu.step(bus)); assert_eq!(0x3F, cpu.mem.r8(0x1001)); assert!(flags(&cpu, HF|NF));
        assert_eq!(23, cpu.step(bus)); assert_eq!(0x80, cpu.mem.r8(0x1002)); assert!(flags(&cpu, SF|HF|VF));
        assert_eq!(23, cpu.step(bus)); assert_eq!(0x7F, cpu.mem.r8(0x1002)); assert!(flags(&cpu, HF|PF|NF));
    }

    #[test]
    fn test_inc_dec_ssixiy() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let prog = [
            0x01, 0x00, 0x00,       // LD BC,0x0000
            0x11, 0xFF, 0xFF,       // LD DE,0xffff
            0x21, 0xFF, 0x00,       // LD HL,0x00ff
            0x31, 0x11, 0x11,       // LD SP,0x1111
            0xDD, 0x21, 0xFF, 0x0F, // LD IX,0x0fff
            0xFD, 0x21, 0x34, 0x12, // LD IY,0x1234
            0x0B,                   // DEC BC
            0x03,                   // INC BC
            0x13,                   // INC DE
            0x1B,                   // DEC DE
            0x23,                   // INC HL
            0x2B,                   // DEC HL
            0x33,                   // INC SP
            0x3B,                   // DEC SP
            0xDD, 0x23,             // INC IX
            0xDD, 0x2B,             // DEC IX
            0xFD, 0x23,             // INC IX
            0xFD, 0x2B,             // DEC IX
        ];
        cpu.mem.write(0x0000, &prog);

        for _ in 0..6 {
            cpu.step(bus);
        }
        assert_eq!(6, cpu.step(bus)); assert_eq!(0xFFFF, cpu.reg.get16(BC));
        assert_eq!(6, cpu.step(bus)); assert_eq!(0x0000, cpu.reg.get16(BC));
        assert_eq!(6, cpu.step(bus)); assert_eq!(0x0000, cpu.reg.get16(DE));
        assert_eq!(6, cpu.step(bus)); assert_eq!(0xFFFF, cpu.reg.get16(DE));
        assert_eq!(6, cpu.step(bus)); assert_eq!(0x0100, cpu.reg.get16(HL));
        assert_eq!(6, cpu.step(bus)); assert_eq!(0x00FF, cpu.reg.get16(HL));
        assert_eq!(6, cpu.step(bus)); assert_eq!(0x1112, cpu.reg.get16(SP));
        assert_eq!(6, cpu.step(bus)); assert_eq!(0x1111, cpu.reg.get16(SP));
        assert_eq!(10, cpu.step(bus)); assert_eq!(0x1000, cpu.reg.get16(IX));
        assert_eq!(10, cpu.step(bus)); assert_eq!(0x0FFF, cpu.reg.get16(IX));
        assert_eq!(10, cpu.step(bus)); assert_eq!(0x1235, cpu.reg.get16(IY));
        assert_eq!(10, cpu.step(bus)); assert_eq!(0x1234, cpu.reg.get16(IY));
    }

    #[test]
    fn test_djnz() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let prog = [
            0x06, 0x03,     // LD BC,0x03
            0x97,           // SUB A
            0x3C,           // loop: INC A
            0x10, 0xFD,     // DJNZ loop
            0x00,           // NOP
        ];
        cpu.mem.write(0x0204, &prog);
        cpu.reg.set_pc(0x0204);

        assert_eq!(7, cpu.step(bus)); assert_eq!(0x03, cpu.reg.get8(B));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x00, cpu.reg.get8(A));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x01, cpu.reg.get8(A));
        assert_eq!(13, cpu.step(bus)); assert_eq!(0x02, cpu.reg.get8(B)); assert_eq!(0x0207, cpu.reg.pc());
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x02, cpu.reg.get8(A));
        assert_eq!(13, cpu.step(bus)); assert_eq!(0x01, cpu.reg.get8(B)); assert_eq!(0x0207, cpu.reg.pc());
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x03, cpu.reg.get8(A));
        assert_eq!(8, cpu.step(bus)); assert_eq!(0x00, cpu.reg.get8(B)); assert_eq!(0x020A, cpu.reg.pc());
    }

    #[test]
    fn test_jr_cc() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let prog = [
            0x97,           //      SUB A
            0x20, 0x03,     //      JR NZ l0
            0x28, 0x01,     //      JR Z, l0
            0x00,           //      NOP
            0xC6, 0x01,     // l0:  ADD A,0x01
            0x28, 0x03,     //      JR Z, l1
            0x20, 0x01,     //      HR NZ, l1
            0x00,           //      NOP
            0xD6, 0x03,     // l1:  SUB 0x03
            0x30, 0x03,     //      JR NC, l2
            0x38, 0x01,     //      JR C, l2
            0x00,           //      NOP
            0x00,           //      NOP
        ];
        cpu.mem.write(0x204, &prog);
        cpu.reg.set_pc(0x0204);

        assert_eq!(4, cpu.step(bus)); assert_eq!(0x00, cpu.reg.get8(A));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x0207, cpu.reg.pc());
        assert_eq!(12, cpu.step(bus)); assert_eq!(0x020A, cpu.reg.pc());
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x01, cpu.reg.get8(A));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x020E, cpu.reg.pc());
        assert_eq!(12, cpu.step(bus)); assert_eq!(0x0211, cpu.reg.pc());
        assert_eq!(7, cpu.step(bus)); assert_eq!(0xFE, cpu.reg.get8(A));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x0215, cpu.reg.pc());
        assert_eq!(12, cpu.step(bus)); assert_eq!(0x0218, cpu.reg.pc());
    }

    #[test]
    fn test_ihl_r() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let prog = [
            0x21, 0x00, 0x10,   // LD HL,0x1000
            0x3E, 0x12,         // LD A,0x12
            0x77,               // LD (HL),A
            0x06, 0x13,         // LD B,0x13
            0x70,               // LD (HL),B
            0x0E, 0x14,         // LD C,0x14
            0x71,               // LD (HL),C
            0x16, 0x15,         // LD D,0x15
            0x72,               // LD (HL),D
            0x1E, 0x16,         // LD E,0x16
            0x73,               // LD (HL),E
            0x74,               // LD (HL),H
            0x75,               // LD (HL),L
        ];
        cpu.mem.write(0x0000, &prog);

        assert_eq!(10, cpu.step(bus)); assert_eq!(0x1000, cpu.reg.get16(HL));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x12, cpu.reg.get8(A));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x12, cpu.mem.r8(0x1000));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x13, cpu.reg.get8(B));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x13, cpu.mem.r8(0x1000));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x14, cpu.reg.get8(C));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x14, cpu.mem.r8(0x1000));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x15, cpu.reg.get8(D));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x15, cpu.mem.r8(0x1000));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x16, cpu.reg.get8(E));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x16, cpu.mem.r8(0x1000));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x10, cpu.mem.r8(0x1000));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x00, cpu.mem.r8(0x1000));
    }

    #[test]
    fn test_inc_dec_ss() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let prog = [
            0x01, 0x00, 0x00,       // LD BC,0x0000
            0x11, 0xFF, 0xFF,       // LD DE,0xffff
            0x21, 0xFF, 0x00,       // LD HL,0x00ff
            0x31, 0x11, 0x11,       // LD SP,0x1111
            0x0B,                   // DEC BC
            0x03,                   // INC BC
            0x13,                   // INC DE
            0x1B,                   // DEC DE
            0x23,                   // INC HL
            0x2B,                   // DEC HL
            0x33,                   // INC SP
            0x3B,                   // DEC SP
        ];
        cpu.mem.write(0x0000, &prog);

        for _ in 0..4 {
            cpu.step(bus);
        }
        assert_eq!(6, cpu.step(bus)); assert_eq!(0xFFFF, cpu.reg.get16(BC));
        assert_eq!(6, cpu.step(bus)); assert_eq!(0x0000, cpu.reg.get16(BC));
        assert_eq!(6, cpu.step(bus)); assert_eq!(0x0000, cpu.reg.get16(DE));
        assert_eq!(6, cpu.step(bus)); assert_eq!(0xFFFF, cpu.reg.get16(DE));
        assert_eq!(6, cpu.step(bus)); assert_eq!(0x0100, cpu.reg.get16(HL));
        assert_eq!(6, cpu.step(bus)); assert_eq!(0x00FF, cpu.reg.get16(HL));
        assert_eq!(6, cpu.step(bus)); assert_eq!(0x1112, cpu.reg.get16(SP));
        assert_eq!(6, cpu.step(bus)); assert_eq!(0x1111, cpu.reg.get16(SP));
    }

    #[test]
    fn test_ld_a_ibcdenn() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let data = [ 0x11, 0x22, 0x33];
        cpu.mem.write(0x1000, &data);
        let prog = [
            0x01, 0x00, 0x10,       // LD BC,0x1000
            0x11, 0x01, 0x10,       // LD DE,0x1001
            0x0A,                   // LD A,(BC)
            0x1A,                   // LD A,(DE)
            0x3A, 0x02, 0x10,       // LD A,(0x1002)
        ];
        cpu.mem.write(0x0000, &prog);

        assert_eq!(10, cpu.step(bus)); assert_eq!(0x1000, cpu.reg.get16(BC));
        assert_eq!(10, cpu.step(bus)); assert_eq!(0x1001, cpu.reg.get16(DE));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x11, cpu.reg.get8(A));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x22, cpu.reg.get8(A));
        assert_eq!(13, cpu.step(bus)); assert_eq!(0x33, cpu.reg.get8(A));
    }

    #[test]
    fn test_ld_ibcdenn_a() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let prog = [
            0x01, 0x00, 0x10,   // LD BC,0x1000
            0x11, 0x01, 0x10,   // LD DE,0x1001
            0x3E, 0x77,         // LD A,0x77
            0x02,               // LD (BC),A
            0x12,               // LD (DE),A
            0x32, 0x02, 0x10,   // LD (0x1002),A
        ];
        cpu.mem.write(0x0000, &prog);

        assert_eq!(10, cpu.step(bus)); assert_eq!(0x1000, cpu.reg.get16(BC));
        assert_eq!(10, cpu.step(bus)); assert_eq!(0x1001, cpu.reg.get16(DE));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x77, cpu.reg.get8(A));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x77, cpu.mem.r8(0x1000));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x77, cpu.mem.r8(0x1001));
        assert_eq!(13, cpu.step(bus)); assert_eq!(0x77, cpu.mem.r8(0x1002));
    }

    #[test]
    fn test_rlca_rla_rrca_rra() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let prog = [
            0x3E, 0xA0,     // LD A,0xA0
            0x07,           // RLCA
            0x07,           // RLCA
            0x0F,           // RRCA
            0x0F,           // RRCA
            0x17,           // RLA
            0x17,           // RLA
            0x1F,           // RRA
            0x1F,           // RRA
        ];
        cpu.mem.write(0x0000, &prog);
        cpu.reg.set8(F, 0xFF);
        assert_eq!(7, cpu.step(bus)); assert_eq!(0xA0, cpu.reg.get8(A));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x41, cpu.reg.get8(A));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x82, cpu.reg.get8(A));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x41, cpu.reg.get8(A));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0xA0, cpu.reg.get8(A));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x41, cpu.reg.get8(A));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x83, cpu.reg.get8(A));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x41, cpu.reg.get8(A));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0xA0, cpu.reg.get8(A));
    }

    #[test]
    fn test_daa() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let prog = [
            0x3E, 0x15,     // LD A,0x15
            0x06, 0x27,     // LD B,0x27
            0x80,           // ADD A,B
            0x27,           // DAA
            0x90,           // SUB B
            0x27,           // DAA
            0x3E, 0x90,     // LD A,0x90
            0x06, 0x15,     // LD B,0x15
            0x80,           // ADD A,B
            0x27,           // DAA
            0x90,           // SUB B
            0x27,           // DAA
        ];
        cpu.mem.write(0x0000, &prog);
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x15, cpu.reg.get8(A));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x27, cpu.reg.get8(B));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x3C, cpu.reg.get8(A)); assert!(flags(&cpu, 0));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x42, cpu.reg.get8(A)); assert!(flags(&cpu, HF|PF));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x1B, cpu.reg.get8(A)); assert!(flags(&cpu, HF|NF));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x15, cpu.reg.get8(A)); assert!(flags(&cpu, NF));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x90, cpu.reg.get8(A)); assert!(flags(&cpu, NF));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x15, cpu.reg.get8(B)); assert!(flags(&cpu, NF));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0xA5, cpu.reg.get8(A)); assert!(flags(&cpu, SF));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x05, cpu.reg.get8(A)); assert!(flags(&cpu, PF|CF));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0xF0, cpu.reg.get8(A)); assert!(flags(&cpu, SF|NF|CF));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x90, cpu.reg.get8(A)); assert!(flags(&cpu, SF|PF|NF|CF));
    }

    #[test]
    fn test_cpl() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let prog = [
            0x97,           // SUB A
            0x2F,           // CPL
            0x2F,           // CPL
            0xC6, 0xAA,     // ADD A,0xAA
            0x2F,           // CPL
            0x2F,           // CPL
        ];
        cpu.mem.write(0x0000, &prog);

        assert_eq!(4, cpu.step(bus)); assert_eq!(0x00, cpu.reg.get8(A)); assert!(flags(&cpu, ZF|NF));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0xFF, cpu.reg.get8(A)); assert!(flags(&cpu, ZF|HF|NF));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x00, cpu.reg.get8(A)); assert!(flags(&cpu, ZF|HF|NF));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0xAA, cpu.reg.get8(A)); assert!(flags(&cpu, SF));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x55, cpu.reg.get8(A)); assert!(flags(&cpu, SF|HF|NF));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0xAA, cpu.reg.get8(A)); assert!(flags(&cpu, SF|HF|NF));
    }

    #[test]
    fn test_ccf_scf() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let prog = [
            0x97,           // SUB A
            0x37,           // SCF
            0x3F,           // CCF
            0xD6, 0xCC,     // SUB 0xCC
            0x3F,           // CCF
            0x37,           // SCF
        ];
        cpu.mem.write(0x0000, &prog);

        assert_eq!(4, cpu.step(bus)); assert_eq!(0x00, cpu.reg.get8(A)); assert!(flags(&cpu, ZF|NF));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x00, cpu.reg.get8(A)); assert!(flags(&cpu, ZF|CF));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x00, cpu.reg.get8(A)); assert!(flags(&cpu, ZF|HF));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x34, cpu.reg.get8(A)); assert!(flags(&cpu, HF|NF|CF));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x34, cpu.reg.get8(A)); assert!(flags(&cpu, HF));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x34, cpu.reg.get8(A)); assert!(flags(&cpu, CF));
    }

    #[test]
    fn test_call_ret() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let prog = [
            0xCD, 0x0A, 0x02,   // CALL l0
            0xCD, 0x0A, 0x02,   // CALL l0
            0xC9,               // l0: RET
        ];
        cpu.mem.write(0x0204, &prog);
        cpu.reg.set_pc(0x0204);

        assert_eq!(17, cpu.step(bus));
        assert_eq!(0x020A, cpu.reg.pc());
        assert_eq!(0xFFFE, cpu.reg.get16(SP));
        assert_eq!(0x0207, cpu.mem.r16(0xFFFE));
        assert_eq!(10, cpu.step(bus));
        assert_eq!(0x0207, cpu.reg.pc());
        assert_eq!(0x0000, cpu.reg.get16(SP));
        assert_eq!(17, cpu.step(bus));
        assert_eq!(0x020A, cpu.reg.pc());
        assert_eq!(0xFFFE, cpu.reg.get16(SP));
        assert_eq!(0x020A, cpu.mem.r16(0xFFFE));
        assert_eq!(10, cpu.step(bus));
        assert_eq!(0x020A, cpu.reg.pc());
        assert_eq!(0x0000, cpu.reg.get16(SP));
    }

    #[test]
    fn test_call_cc_ret_cc() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let prog = [
			0x97,               //      SUB A
			0xC4, 0x29, 0x02,   //      CALL NZ,l0
			0xCC, 0x29, 0x02,   //      CALL Z,l0
			0xC6, 0x01,         //      ADD A,0x01
			0xCC, 0x2B, 0x02,   //      CALL Z,l1
			0xC4, 0x2B, 0x02,   //      CALL NZ,l1
			0x07,               //      RLCA
			0xEC, 0x2D, 0x02,   //      CALL PE,l2
			0xE4, 0x2D, 0x02,   //      CALL PO,l2
			0xD6, 0x03,         //      SUB 0x03
			0xF4, 0x2F, 0x02,   //      CALL P,l3
			0xFC, 0x2F, 0x02,   //      CALL M,l3
			0xD4, 0x31, 0x02,   //      CALL NC,l4
			0xDC, 0x31, 0x02,   //      CALL C,l4
			0xC9,               //      RET
			0xC0,               // l0:  RET NZ
			0xC8,               //      RET Z
			0xC8,               // l1:  RET Z
			0xC0,               //      RET NZ
			0xE8,               // l2:  RET PE
			0xE0,               //      RET PO
			0xF0,               // l3:  RET P
			0xF8,               //      RET M
			0xD0,               // l4:  RET NC
			0xD8,               //      RET C<Paste>
        ];
		cpu.mem.write(0x0204, &prog);
		cpu.reg.set_pc(0x0204);
		cpu.reg.set16(SP, 0x0100);

        assert_eq!(4, cpu.step(bus)); assert_eq!(0x00, cpu.reg.get8(A));
        assert_eq!(10, cpu.step(bus)); assert_eq!(0x0208, cpu.reg.pc());
        assert_eq!(17, cpu.step(bus)); assert_eq!(0x0229, cpu.reg.pc());
        assert_eq!(5, cpu.step(bus)); assert_eq!(0x022A, cpu.reg.pc());
        assert_eq!(11, cpu.step(bus)); assert_eq!(0x020B, cpu.reg.pc());
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x01, cpu.reg.get8(A));
        assert_eq!(10, cpu.step(bus)); assert_eq!(0x0210, cpu.reg.pc());
        assert_eq!(17, cpu.step(bus)); assert_eq!(0x022B, cpu.reg.pc());
        assert_eq!(5, cpu.step(bus)); assert_eq!(0x022C, cpu.reg.pc());
        assert_eq!(11, cpu.step(bus)); assert_eq!(0x0213, cpu.reg.pc());
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x02, cpu.reg.get8(A));
        assert_eq!(10, cpu.step(bus)); assert_eq!(0x0217, cpu.reg.pc());
        assert_eq!(17, cpu.step(bus)); assert_eq!(0x022D, cpu.reg.pc());
        assert_eq!(5, cpu.step(bus)); assert_eq!(0x022E, cpu.reg.pc());
        assert_eq!(11, cpu.step(bus)); assert_eq!(0x021A, cpu.reg.pc());
        assert_eq!(7, cpu.step(bus)); assert_eq!(0xFF, cpu.reg.get8(A));
        assert_eq!(10, cpu.step(bus)); assert_eq!(0x021F, cpu.reg.pc());
        assert_eq!(17, cpu.step(bus)); assert_eq!(0x022F, cpu.reg.pc());
        assert_eq!(5, cpu.step(bus)); assert_eq!(0x0230, cpu.reg.pc());
        assert_eq!(11, cpu.step(bus)); assert_eq!(0x0222, cpu.reg.pc());
        assert_eq!(10, cpu.step(bus)); assert_eq!(0x0225, cpu.reg.pc());
        assert_eq!(17, cpu.step(bus)); assert_eq!(0x0231, cpu.reg.pc());
        assert_eq!(5, cpu.step(bus)); assert_eq!(0x0232, cpu.reg.pc());
        assert_eq!(11, cpu.step(bus)); assert_eq!(0x0228, cpu.reg.pc());
    }

    #[test]
    fn test_halt() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let prog = [
            0x76,       // HALT
        ];
        cpu.mem.write(0x0000, &prog);
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x0000, cpu.reg.pc()); assert!(cpu.halt);
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x0000, cpu.reg.pc()); assert!(cpu.halt);
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x0000, cpu.reg.pc()); assert!(cpu.halt);
    }

    #[test]
    fn test_ex() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let prog = [
            0x21, 0x34, 0x12,       // LD HL,0x1234
            0x11, 0x78, 0x56,       // LD DE,0x5678
            0xEB,                   // EX DE,HL
            0x3E, 0x11,             // LD A,0x11
            0x08,                   // EX AF,AF'
            0x3E, 0x22,             // LD A,0x22
            0x08,                   // EX AF,AF'
            0x01, 0xBC, 0x9A,       // LD BC,0x9ABC
            0xD9,                   // EXX
            0x21, 0x11, 0x11,       // LD HL,0x1111
            0x11, 0x22, 0x22,       // LD DE,0x2222
            0x01, 0x33, 0x33,       // LD BC,0x3333
            0xD9,                   // EXX
            0x31, 0x00, 0x01,       // LD SP,0x0100
            0xD5,                   // PUSH DE
            0xE3,                   // EX (SP),HL
            0xDD, 0x21, 0x99, 0x88, // LD IX,0x8899
            0xDD, 0xE3,             // EX (SP),IX
            0xFD, 0x21, 0x77, 0x66, // LD IY,0x6677
            0xFD, 0xE3,             // EX (SP),IY
        ];
        cpu.mem.write(0x0000, &prog);

        assert_eq!(10, cpu.step(bus)); assert_eq!(0x1234, cpu.reg.get16(HL));
        assert_eq!(10, cpu.step(bus)); assert_eq!(0x5678, cpu.reg.get16(DE));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x1234, cpu.reg.get16(DE)); assert_eq!(0x5678, cpu.reg.get16(HL));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x1100, cpu.reg.get16(AF)); assert_eq!(0x0000, cpu.reg.get16(AF_));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x0000, cpu.reg.get16(AF)); assert_eq!(0x1100, cpu.reg.get16(AF_));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x2200, cpu.reg.get16(AF)); assert_eq!(0x1100, cpu.reg.get16(AF_));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x1100, cpu.reg.get16(AF)); assert_eq!(0x2200, cpu.reg.get16(AF_));
        assert_eq!(10, cpu.step(bus)); assert_eq!(0x9ABC, cpu.reg.get16(BC));
        assert_eq!(4, cpu.step(bus));
        assert_eq!(0x0000, cpu.reg.get16(HL)); assert_eq!(0x5678, cpu.reg.get16(HL_));
        assert_eq!(0x0000, cpu.reg.get16(DE)); assert_eq!(0x1234, cpu.reg.get16(DE_));
        assert_eq!(0x0000, cpu.reg.get16(BC)); assert_eq!(0x9ABC, cpu.reg.get16(BC_));
        assert_eq!(10, cpu.step(bus)); assert_eq!(0x1111, cpu.reg.get16(HL));
        assert_eq!(10, cpu.step(bus)); assert_eq!(0x2222, cpu.reg.get16(DE));
        assert_eq!(10, cpu.step(bus)); assert_eq!(0x3333, cpu.reg.get16(BC));
        assert_eq!(4, cpu.step(bus));
        assert_eq!(0x5678, cpu.reg.get16(HL)); assert_eq!(0x1111, cpu.reg.get16(HL_));
        assert_eq!(0x1234, cpu.reg.get16(DE)); assert_eq!(0x2222, cpu.reg.get16(DE_));
        assert_eq!(0x9ABC, cpu.reg.get16(BC)); assert_eq!(0x3333, cpu.reg.get16(BC_));
        assert_eq!(10, cpu.step(bus)); assert_eq!(0x0100, cpu.reg.get16(SP));
        assert_eq!(11, cpu.step(bus)); assert_eq!(0x1234, cpu.mem.r16(0x00FE));
        assert_eq!(19, cpu.step(bus)); assert_eq!(0x1234, cpu.reg.get16(HL)); assert_eq!(0x5678, cpu.mem.r16(0x00FE));
        assert_eq!(14, cpu.step(bus)); assert_eq!(0x8899, cpu.reg.get16(IX));
        assert_eq!(23, cpu.step(bus)); assert_eq!(0x5678, cpu.reg.get16(IX)); assert_eq!(0x8899, cpu.mem.r16(0x00FE));
        assert_eq!(14, cpu.step(bus)); assert_eq!(0x6677, cpu.reg.get16(IY));
        assert_eq!(23, cpu.step(bus)); assert_eq!(0x8899, cpu.reg.get16(IY)); assert_eq!(0x6677, cpu.mem.r16(0x00FE));
    }

    #[test]
    fn test_jp_cc_nn() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let prog = [
            0x97,               //          SUB A
            0xC2, 0x0C, 0x02,   //          JP NZ,label0
            0xCA, 0x0C, 0x02,   //          JP Z,label0
            0x00,               //          NOP
            0xC6, 0x01,         // label0:  ADD A,0x01
            0xCA, 0x15, 0x02,   //          JP Z,label1
            0xC2, 0x15, 0x02,   //          JP NZ,label1
            0x00,               //          NOP
            0x07,               // label1:  RLCA
            0xEA, 0x1D, 0x02,   //          JP PE,label2
            0xE2, 0x1D, 0x02,   //          JP PO,label2
            0x00,               //          NOP
            0xC6, 0xFD,         // label2:  ADD A,0xFD
            0xF2, 0x26, 0x02,   //          JP P,label3
            0xFA, 0x26, 0x02,   //          JP M,label3
            0x00,               //          NOP
            0xD2, 0x2D, 0x02,   // label3:  JP NC,label4
            0xDA, 0x2D, 0x02,   //          JP C,label4
            0x00,               //          NOP
            0x00,               //          NOP
        ];
        cpu.mem.write(0x0204, &prog);
        cpu.reg.set_pc(0x0204);

        assert_eq!(4, cpu.step(bus)); assert_eq!(0x00, cpu.reg.get8(A)); assert!(flags(&cpu, ZF|NF));
        assert_eq!(10, cpu.step(bus)); assert_eq!(0x0208, cpu.reg.pc());
        assert_eq!(10, cpu.step(bus)); assert_eq!(0x020C, cpu.reg.pc());
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x01, cpu.reg.get8(A)); assert!(flags(&cpu, 0));
        assert_eq!(10, cpu.step(bus)); assert_eq!(0x0211, cpu.reg.pc());
        assert_eq!(10, cpu.step(bus)); assert_eq!(0x0215, cpu.reg.pc());
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x02, cpu.reg.get8(A)); assert!(flags(&cpu, 0));
        assert_eq!(10, cpu.step(bus)); assert_eq!(0x0219, cpu.reg.pc());
        assert_eq!(10, cpu.step(bus)); assert_eq!(0x021D, cpu.reg.pc());
        assert_eq!(7, cpu.step(bus)); assert_eq!(0xFF, cpu.reg.get8(A)); assert!(flags(&cpu, SF));
        assert_eq!(10, cpu.step(bus)); assert_eq!(0x0222, cpu.reg.pc());
        assert_eq!(10, cpu.step(bus)); assert_eq!(0x0226, cpu.reg.pc());
        assert_eq!(10, cpu.step(bus)); assert_eq!(0x022D, cpu.reg.pc());
    }

    #[test]
    fn test_jp_jr() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let prog = [
            0x21, 0x16, 0x02,           //      LD HL,l3
            0xDD, 0x21, 0x19, 0x02,     //      LD IX,l4
            0xFD, 0x21, 0x21, 0x02,     //      LD IY,l5
            0xC3, 0x14, 0x02,           //      JP l0
            0x18, 0x04,                 // l1:  JR l2
            0x18, 0xFC,                 // l0:  JR l1
            0xDD, 0xE9,                 // l3:  JP (IX)
            0xE9,                       // l2:  JP (HL)
            0xFD, 0xE9,                 // l4:  JP (IY)
            0x18, 0x06,                 // l6:  JR l7
            0x00, 0x00, 0x00, 0x00,     //      4x NOP
            0x18, 0xF8,                 // l5:  JR l6
            0x00                        // l7:  NOP
        ];
        cpu.mem.write(0x0204, &prog);
        cpu.reg.set_pc(0x0204);

        assert_eq!(10, cpu.step(bus)); assert_eq!(0x0216, cpu.reg.get16(HL));
        assert_eq!(14, cpu.step(bus)); assert_eq!(0x0219, cpu.reg.get16(IX));
        assert_eq!(14, cpu.step(bus)); assert_eq!(0x0221, cpu.reg.get16(IY));
        assert_eq!(10, cpu.step(bus)); assert_eq!(0x0214, cpu.reg.pc());
        assert_eq!(12, cpu.step(bus)); assert_eq!(0x0212, cpu.reg.pc());
        assert_eq!(12, cpu.step(bus)); assert_eq!(0x0218, cpu.reg.pc());
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x0216, cpu.reg.pc());
        assert_eq!(8, cpu.step(bus)); assert_eq!(0x0219, cpu.reg.pc());
        assert_eq!(8, cpu.step(bus)); assert_eq!(0x0221, cpu.reg.pc());
        assert_eq!(12, cpu.step(bus)); assert_eq!(0x021B, cpu.reg.pc());
        assert_eq!(12, cpu.step(bus)); assert_eq!(0x0223, cpu.reg.pc());
    }

    #[test]
    fn test_ldi() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let data = [ 0x01, 0x02, 0x03 ];
        cpu.mem.write(0x1000, &data);
        let prog = [
            0x21, 0x00, 0x10,       // LD HL,0x1000
            0x11, 0x00, 0x20,       // LD DE,0x2000
            0x01, 0x03, 0x00,       // LD BC,0x0003
            0xED, 0xA0,             // LDI
            0xED, 0xA0,             // LDI
            0xED, 0xA0,             // LDI
        ];
        cpu.mem.write(0x0000, &prog);

        // skip loads
        for _ in 0..3 {
            cpu.step(bus);
        }
        assert_eq!(16, cpu.step(bus));
        assert_eq!(0x1001, cpu.reg.get16(HL));
        assert_eq!(0x2001, cpu.reg.get16(DE));
        assert_eq!(0x0002, cpu.reg.get16(BC));
        assert_eq!(0x01, cpu.mem.r8(0x2000));
        assert!(flags(&cpu, PF));
        assert_eq!(16, cpu.step(bus));
        assert_eq!(0x1002, cpu.reg.get16(HL));
        assert_eq!(0x2002, cpu.reg.get16(DE));
        assert_eq!(0x0001, cpu.reg.get16(BC));
        assert_eq!(0x02, cpu.mem.r8(0x2001));
        assert!(flags(&cpu, PF));
        assert_eq!(16, cpu.step(bus));
        assert_eq!(0x1003, cpu.reg.get16(HL));
        assert_eq!(0x2003, cpu.reg.get16(DE));
        assert_eq!(0x0000, cpu.reg.get16(BC));
        assert_eq!(0x03, cpu.mem.r8(0x2002));
        assert!(flags(&cpu, 0));
    }

    #[test]
    fn test_ldir() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let data = [ 0x01, 0x02, 0x03 ];
        cpu.mem.write(0x1000, &data);
        let prog = [
            0x21, 0x00, 0x10,       // LD HL,0x1000
            0x11, 0x00, 0x20,       // LD DE,0x2000
            0x01, 0x03, 0x00,       // LD BC,0x0003
            0xED, 0xB0,             // LDIR
            0x3E, 0x33,             // LD A,0x33
        ];
        cpu.mem.write(0x0000, &prog);

        // skip loads
        for _ in 0..3 {
            cpu.step(bus);
        }
        assert_eq!(21, cpu.step(bus));
        assert_eq!(0x1001, cpu.reg.get16(HL));
        assert_eq!(0x2001, cpu.reg.get16(DE));
        assert_eq!(0x0002, cpu.reg.get16(BC));
        assert_eq!(0x01, cpu.mem.r8(0x2000));
        assert!(flags(&cpu, PF));
        assert_eq!(21, cpu.step(bus));
        assert_eq!(0x1002, cpu.reg.get16(HL));
        assert_eq!(0x2002, cpu.reg.get16(DE));
        assert_eq!(0x0001, cpu.reg.get16(BC));
        assert_eq!(0x02, cpu.mem.r8(0x2001));
        assert!(flags(&cpu, PF));
        assert_eq!(16, cpu.step(bus));
        assert_eq!(0x1003, cpu.reg.get16(HL));
        assert_eq!(0x2003, cpu.reg.get16(DE));
        assert_eq!(0x0000, cpu.reg.get16(BC));
        assert_eq!(0x03, cpu.mem.r8(0x2002));
        assert!(flags(&cpu, 0));
        cpu.step(bus); assert_eq!(0x33, cpu.reg.get8(A));
    }

    #[test]
    fn test_ldd() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let data = [ 0x01, 0x02, 0x03 ];
        cpu.mem.write(0x1000, &data);
        let prog = [
            0x21, 0x02, 0x10,       // LD HL,0x1002
            0x11, 0x02, 0x20,       // LD DE,0x2002
            0x01, 0x03, 0x00,       // LD BC,0x0003
            0xED, 0xA8,             // LDD
            0xED, 0xA8,             // LDD
            0xED, 0xA8,             // LDD
        ];
        cpu.mem.write(0x0000, &prog);

        // skip loads
        for _ in 0..3 {
            cpu.step(bus);
        }
        assert_eq!(16, cpu.step(bus));
        assert_eq!(0x1001, cpu.reg.get16(HL));
        assert_eq!(0x2001, cpu.reg.get16(DE));
        assert_eq!(0x0002, cpu.reg.get16(BC));
        assert_eq!(0x03, cpu.mem.r8(0x2002));
        assert!(flags(&cpu, PF));
        assert_eq!(16, cpu.step(bus));
        assert_eq!(0x1000, cpu.reg.get16(HL));
        assert_eq!(0x2000, cpu.reg.get16(DE));
        assert_eq!(0x0001, cpu.reg.get16(BC));
        assert_eq!(0x02, cpu.mem.r8(0x2001));
        assert!(flags(&cpu, PF));
        assert_eq!(16, cpu.step(bus));
        assert_eq!(0x0FFF, cpu.reg.get16(HL));
        assert_eq!(0x1FFF, cpu.reg.get16(DE));
        assert_eq!(0x0000, cpu.reg.get16(BC));
        assert_eq!(0x01, cpu.mem.r8(0x2000));
        assert!(flags(&cpu, 0));
    }

    #[test]
    fn test_lddr() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let data = [ 0x01, 0x02, 0x03 ];
        cpu.mem.write(0x1000, &data);
        let prog = [
            0x21, 0x02, 0x10,       // LD HL,0x1002
            0x11, 0x02, 0x20,       // LD DE,0x2002
            0x01, 0x03, 0x00,       // LD BC,0x0003
            0xED, 0xB8,             // LDDR
            0x3E, 0x33,             // LD A,0x33
        ];
        cpu.mem.write(0x0000, &prog);

        // skip loads
        for _ in 0..3 {
            cpu.step(bus);
        }
        assert_eq!(21, cpu.step(bus));
        assert_eq!(0x1001, cpu.reg.get16(HL));
        assert_eq!(0x2001, cpu.reg.get16(DE));
        assert_eq!(0x0002, cpu.reg.get16(BC));
        assert_eq!(0x03, cpu.mem.r8(0x2002));
        assert!(flags(&cpu, PF));
        assert_eq!(21, cpu.step(bus));
        assert_eq!(0x1000, cpu.reg.get16(HL));
        assert_eq!(0x2000, cpu.reg.get16(DE));
        assert_eq!(0x0001, cpu.reg.get16(BC));
        assert_eq!(0x02, cpu.mem.r8(0x2001));
        assert!(flags(&cpu, PF));
        assert_eq!(16, cpu.step(bus));
        assert_eq!(0x0FFF, cpu.reg.get16(HL));
        assert_eq!(0x1FFF, cpu.reg.get16(DE));
        assert_eq!(0x0000, cpu.reg.get16(BC));
        assert_eq!(0x01, cpu.mem.r8(0x2000));
        assert!(flags(&cpu, 0));
        cpu.step(bus); assert_eq!(0x33, cpu.reg.get8(A));
    }

    #[test]
    fn test_cpi() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let data = [ 0x01, 0x02, 0x03, 0x04 ];
        cpu.mem.write(0x1000, &data);
        let prog = [
            0x21, 0x00, 0x10,       // ld hl,0x1000
            0x01, 0x04, 0x00,       // ld bc,0x0004
            0x3e, 0x03,             // ld a,0x03
            0xed, 0xa1,             // cpi
            0xed, 0xa1,             // cpi
            0xed, 0xa1,             // cpi
            0xed, 0xa1,             // cpi
        ];
        cpu.mem.write(0x0000, &prog);

        // skip loads
        for _ in 0..3 {
            cpu.step(bus);
        }
        assert_eq!(16, cpu.step(bus));
        assert_eq!(0x1001, cpu.reg.get16(HL));
        assert_eq!(0x0003, cpu.reg.get16(BC));
        assert!(flags(&cpu, PF|NF));
        let f = cpu.reg.get8(F) | CF;
        cpu.reg.set8(F, f);
        assert_eq!(16, cpu.step(bus));
        assert_eq!(0x1002, cpu.reg.get16(HL));
        assert_eq!(0x0002, cpu.reg.get16(BC));
        assert!(flags(&cpu, PF|NF|CF));
        assert_eq!(16, cpu.step(bus));
        assert_eq!(0x1003, cpu.reg.get16(HL));
        assert_eq!(0x0001, cpu.reg.get16(BC));
        assert!(flags(&cpu, ZF|PF|NF|CF));
        assert_eq!(16, cpu.step(bus));
        assert_eq!(0x1004, cpu.reg.get16(HL));
        assert_eq!(0x0000, cpu.reg.get16(BC));
        assert!(flags(&cpu, SF|HF|NF|CF));
    }

    #[test]
    fn test_cpir() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let data = [ 0x01, 0x02, 0x03, 0x04 ];
        cpu.mem.write(0x1000, &data);
        let prog = [
            0x21, 0x00, 0x10,       // ld hl,0x1000
            0x01, 0x04, 0x00,       // ld bc,0x0004
            0x3e, 0x03,             // ld a,0x03
            0xed, 0xb1,             // cpir
            0xed, 0xb1,             // cpir
        ];
        cpu.mem.write(0x0000, &prog);

        // skip loads
        for _ in 0..3 {
            cpu.step(bus);
        }
        assert_eq!(21, cpu.step(bus));
        assert_eq!(0x1001, cpu.reg.get16(HL));
        assert_eq!(0x0003, cpu.reg.get16(BC));
        assert!(flags(&cpu, PF|NF));
        let f = cpu.reg.get8(F) | CF;
        cpu.reg.set8(F, f);
        assert_eq!(21, cpu.step(bus));
        assert_eq!(0x1002, cpu.reg.get16(HL));
        assert_eq!(0x0002, cpu.reg.get16(BC));
        assert!(flags(&cpu, PF|NF|CF));
        assert_eq!(16, cpu.step(bus));
        assert_eq!(0x1003, cpu.reg.get16(HL));
        assert_eq!(0x0001, cpu.reg.get16(BC));
        assert!(flags(&cpu, ZF|PF|NF|CF));
        assert_eq!(16, cpu.step(bus));
        assert_eq!(0x1004, cpu.reg.get16(HL));
        assert_eq!(0x0000, cpu.reg.get16(BC));
        assert!(flags(&cpu, SF|HF|NF|CF));
    }

    #[test]
    fn test_cpd() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let data = [ 0x01, 0x02, 0x03, 0x04 ];
        cpu.mem.write(0x1000, &data);
        let prog = [
            0x21, 0x03, 0x10,       // ld hl,0x1004
            0x01, 0x04, 0x00,       // ld bc,0x0004
            0x3e, 0x02,             // ld a,0x03
            0xed, 0xa9,             // cpi
            0xed, 0xa9,             // cpi
            0xed, 0xa9,             // cpi
            0xed, 0xa9,             // cpi
        ];
        cpu.mem.write(0x0000, &prog);

        // skip loads
        for _ in 0..3 {
            cpu.step(bus);
        }
        assert_eq!(16, cpu.step(bus));
        assert_eq!(0x1002, cpu.reg.get16(HL));
        assert_eq!(0x0003, cpu.reg.get16(BC));
        assert!(flags(&cpu, SF|HF|PF|NF));
        let f = cpu.reg.get8(F) | CF;
        cpu.reg.set8(F, f);
        assert_eq!(16, cpu.step(bus));
        assert_eq!(0x1001, cpu.reg.get16(HL));
        assert_eq!(0x0002, cpu.reg.get16(BC));
        assert!(flags(&cpu, SF|HF|PF|NF|CF));
        assert_eq!(16, cpu.step(bus));
        assert_eq!(0x1000, cpu.reg.get16(HL));
        assert_eq!(0x0001, cpu.reg.get16(BC));
        assert!(flags(&cpu, ZF|PF|NF|CF));
        assert_eq!(16, cpu.step(bus));
        assert_eq!(0x0FFF, cpu.reg.get16(HL));
        assert_eq!(0x0000, cpu.reg.get16(BC));
        assert!(flags(&cpu, NF|CF));
    }

    #[test]
    fn test_cpdr() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let data = [ 0x01, 0x02, 0x03, 0x04 ];
        cpu.mem.write(0x1000, &data);
        let prog = [
            0x21, 0x03, 0x10,       // ld hl,0x1004
            0x01, 0x04, 0x00,       // ld bc,0x0004
            0x3e, 0x02,             // ld a,0x03
            0xed, 0xb9,             // cpdr
            0xed, 0xb9,             // cpdr
        ];
        cpu.mem.write(0x0000, &prog);

        // skip loads
        for _ in 0..3 {
            cpu.step(bus);
        }
        assert_eq!(21, cpu.step(bus));
        assert_eq!(0x1002, cpu.reg.get16(HL));
        assert_eq!(0x0003, cpu.reg.get16(BC));
        assert!(flags(&cpu, SF|HF|PF|NF));
        let f = cpu.reg.get8(F) | CF;
        cpu.reg.set8(F, f);
        assert_eq!(21, cpu.step(bus));
        assert_eq!(0x1001, cpu.reg.get16(HL));
        assert_eq!(0x0002, cpu.reg.get16(BC));
        assert!(flags(&cpu, SF|HF|PF|NF|CF));
        assert_eq!(16, cpu.step(bus));
        assert_eq!(0x1000, cpu.reg.get16(HL));
        assert_eq!(0x0001, cpu.reg.get16(BC));
        assert!(flags(&cpu, ZF|PF|NF|CF));
        assert_eq!(16, cpu.step(bus));
        assert_eq!(0x0FFF, cpu.reg.get16(HL));
        assert_eq!(0x0000, cpu.reg.get16(BC));
        assert!(flags(&cpu, NF|CF));
    }

    #[test]
    fn test_add_adc_sbc_16() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let prog = [
            0x21, 0xFC, 0x00,       // LD HL,0x00FC
            0x01, 0x08, 0x00,       // LD BC,0x0008
            0x11, 0xFF, 0xFF,       // LD DE,0xFFFF
            0x09,                   // ADD HL,BC
            0x19,                   // ADD HL,DE
            0xED, 0x4A,             // ADC HL,BC
            0x29,                   // ADD HL,HL
            0x19,                   // ADD HL,DE
            0xED, 0x42,             // SBD HL,BC
            0xDD, 0x21, 0xFC, 0x00, // LD IX,0x00FC
            0x31, 0x00, 0x10,       // LD SP,0x1000
            0xDD, 0x09,             // ADD IX, BC
            0xDD, 0x19,             // ADD IX, DE
            0xDD, 0x29,             // ADD IX, IX
            0xDD, 0x39,             // ADD IX, SP
            0xFD, 0x21, 0xFF, 0xFF, // LD IY,0xFFFF
            0xFD, 0x09,             // ADD IY,BC
            0xFD, 0x19,             // ADD IY,DE
            0xFD, 0x29,             // ADD IY,IY
            0xFD, 0x39,             // ADD IY,SP
        ];
        cpu.mem.write(0x0000, &prog);

        assert_eq!(10, cpu.step(bus)); assert_eq!(0x00FC, cpu.reg.get16(HL));
        assert_eq!(10, cpu.step(bus)); assert_eq!(0x0008, cpu.reg.get16(BC));
        assert_eq!(10, cpu.step(bus)); assert_eq!(0xFFFF, cpu.reg.get16(DE));
        assert_eq!(11, cpu.step(bus)); assert_eq!(0x0104, cpu.reg.get16(HL)); assert!(flags(&cpu, 0));
        assert_eq!(11, cpu.step(bus)); assert_eq!(0x0103, cpu.reg.get16(HL)); assert!(flags(&cpu, HF|CF));
        assert_eq!(15, cpu.step(bus)); assert_eq!(0x010C, cpu.reg.get16(HL)); assert!(flags(&cpu, 0));
        assert_eq!(11, cpu.step(bus)); assert_eq!(0x0218, cpu.reg.get16(HL)); assert!(flags(&cpu, 0));
        assert_eq!(11, cpu.step(bus)); assert_eq!(0x0217, cpu.reg.get16(HL)); assert!(flags(&cpu, HF|CF));
        assert_eq!(15, cpu.step(bus)); assert_eq!(0x020E, cpu.reg.get16(HL)); assert!(flags(&cpu, NF));
        assert_eq!(14, cpu.step(bus)); assert_eq!(0x00FC, cpu.reg.get16(IX));
        assert_eq!(10, cpu.step(bus)); assert_eq!(0x1000, cpu.reg.get16(SP));
        assert_eq!(15, cpu.step(bus)); assert_eq!(0x0104, cpu.reg.get16(IX)); assert!(flags(&cpu, 0));
        assert_eq!(15, cpu.step(bus)); assert_eq!(0x0103, cpu.reg.get16(IX)); assert!(flags(&cpu, HF|CF));
        assert_eq!(15, cpu.step(bus)); assert_eq!(0x0206, cpu.reg.get16(IX)); assert!(flags(&cpu, 0));
        assert_eq!(15, cpu.step(bus)); assert_eq!(0x1206, cpu.reg.get16(IX)); assert!(flags(&cpu, 0));
        assert_eq!(14, cpu.step(bus)); assert_eq!(0xFFFF, cpu.reg.get16(IY));
        assert_eq!(15, cpu.step(bus)); assert_eq!(0x0007, cpu.reg.get16(IY)); assert!(flags(&cpu, HF|CF));
        assert_eq!(15, cpu.step(bus)); assert_eq!(0x0006, cpu.reg.get16(IY)); assert!(flags(&cpu, HF|CF));
        assert_eq!(15, cpu.step(bus)); assert_eq!(0x000C, cpu.reg.get16(IY)); assert!(flags(&cpu, 0));
        assert_eq!(15, cpu.step(bus)); assert_eq!(0x100C, cpu.reg.get16(IY)); assert!(flags(&cpu, 0));
    }

    #[test]
    fn ld_hlddixiy_inn() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let data = [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08
        ];
        cpu.mem.write(0x1000, &data);
        let prog = [
            0x2A, 0x00, 0x10,           // LD HL,(0x1000)
            0xED, 0x4B, 0x01, 0x10,     // LD BC,(0x1001)
            0xED, 0x5B, 0x02, 0x10,     // LD DE,(0x1002)
            0xED, 0x6B, 0x03, 0x10,     // LD HL,(0x1003) undocumented 'long' version
            0xED, 0x7B, 0x04, 0x10,     // LD SP,(0x1004)
            0xDD, 0x2A, 0x05, 0x10,     // LD IX,(0x1004)
            0xFD, 0x2A, 0x06, 0x10,     // LD IY,(0x1005)
        ];
        cpu.mem.write(0x0000, &prog);

        assert_eq!(16, cpu.step(bus)); assert_eq!(0x0201, cpu.reg.get16(HL));
        assert_eq!(20, cpu.step(bus)); assert_eq!(0x0302, cpu.reg.get16(BC));
        assert_eq!(20, cpu.step(bus)); assert_eq!(0x0403, cpu.reg.get16(DE));
        assert_eq!(20, cpu.step(bus)); assert_eq!(0x0504, cpu.reg.get16(HL));
        assert_eq!(20, cpu.step(bus)); assert_eq!(0x0605, cpu.reg.get16(SP));
        assert_eq!(20, cpu.step(bus)); assert_eq!(0x0706, cpu.reg.get16(IX));
        assert_eq!(20, cpu.step(bus)); assert_eq!(0x0807, cpu.reg.get16(IY));
    }

    #[test]
    fn ld_inn_hlddixiy() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let prog = [
            0x21, 0x01, 0x02,           // LD HL,0x0201
            0x22, 0x00, 0x10,           // LD (0x1000),HL
            0x01, 0x34, 0x12,           // LD BC,0x1234
            0xED, 0x43, 0x02, 0x10,     // LD (0x1002),BC
            0x11, 0x78, 0x56,           // LD DE,0x5678
            0xED, 0x53, 0x04, 0x10,     // LD (0x1004),DE
            0x21, 0xBC, 0x9A,           // LD HL,0x9ABC
            0xED, 0x63, 0x06, 0x10,     // LD (0x1006),HL undocumented 'long' version
            0x31, 0x68, 0x13,           // LD SP,0x1368
            0xED, 0x73, 0x08, 0x10,     // LD (0x1008),SP
            0xDD, 0x21, 0x21, 0x43,     // LD IX,0x4321
            0xDD, 0x22, 0x0A, 0x10,     // LD (0x100A),IX
            0xFD, 0x21, 0x65, 0x87,     // LD IY,0x8765
            0xFD, 0x22, 0x0C, 0x10,     // LD (0x100C),IY
        ];
        cpu.mem.write(0x0000, &prog);

        assert_eq!(10, cpu.step(bus)); assert_eq!(0x0201, cpu.reg.get16(HL));
        assert_eq!(16, cpu.step(bus)); assert_eq!(0x0201, cpu.mem.r16(0x1000));
        assert_eq!(10, cpu.step(bus)); assert_eq!(0x1234, cpu.reg.get16(BC));
        assert_eq!(20, cpu.step(bus)); assert_eq!(0x1234, cpu.mem.r16(0x1002));
        assert_eq!(10, cpu.step(bus)); assert_eq!(0x5678, cpu.reg.get16(DE));
        assert_eq!(20, cpu.step(bus)); assert_eq!(0x5678, cpu.mem.r16(0x1004));
        assert_eq!(10, cpu.step(bus)); assert_eq!(0x9ABC, cpu.reg.get16(HL));
        assert_eq!(20, cpu.step(bus)); assert_eq!(0x9ABC, cpu.mem.r16(0x1006));
        assert_eq!(10, cpu.step(bus)); assert_eq!(0x1368, cpu.reg.get16(SP));
        assert_eq!(20, cpu.step(bus)); assert_eq!(0x1368, cpu.mem.r16(0x1008));
        assert_eq!(14, cpu.step(bus)); assert_eq!(0x4321, cpu.reg.get16(IX));
        assert_eq!(20, cpu.step(bus)); assert_eq!(0x4321, cpu.mem.r16(0x100A));
        assert_eq!(14, cpu.step(bus)); assert_eq!(0x8765, cpu.reg.get16(IY));
        assert_eq!(20, cpu.step(bus)); assert_eq!(0x8765, cpu.mem.r16(0x100C));
    }

    #[test]
    fn test_neg() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let prog = [
            0x3E, 0x01,         // LD A,0x01
            0xED, 0x44,         // NEG
            0xC6, 0x01,         // ADD A,0x01
            0xED, 0x44,         // NEG
            0xD6, 0x80,         // SUB A,0x80
            0xED, 0x44,         // NEG
            0xC6, 0x40,         // ADD A,0x40
            0xED, 0x44,         // NEG
        ];
        cpu.mem.write(0x0000, &prog);

        assert_eq!(7, cpu.step(bus)); assert_eq!(0x01, cpu.reg.get8(A));
        assert_eq!(8, cpu.step(bus)); assert_eq!(0xFF, cpu.reg.get8(A)); assert!(flags(&cpu, SF|HF|NF|CF));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x00, cpu.reg.get8(A)); assert!(flags(&cpu, ZF|HF|CF));
        assert_eq!(8, cpu.step(bus)); assert_eq!(0x00, cpu.reg.get8(A)); assert!(flags(&cpu, ZF|NF));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x80, cpu.reg.get8(A)); assert!(flags(&cpu, SF|PF|NF|CF));
        assert_eq!(8, cpu.step(bus)); assert_eq!(0x80, cpu.reg.get8(A)); assert!(flags(&cpu, SF|PF|NF|CF));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0xC0, cpu.reg.get8(A)); assert!(flags(&cpu, SF));
        assert_eq!(8, cpu.step(bus)); assert_eq!(0x40, cpu.reg.get8(A)); assert!(flags(&cpu, NF|CF));
    }

    #[test]
    fn test_ld_a_ir() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        cpu.iff1 = true;
        cpu.iff2 = true;
        cpu.reg.r = 0x34;
        cpu.reg.i = 0x1;
        cpu.reg.set8(F, CF);
        let prog = [
            0xED, 0x57,         // LD A,I
            0x97,               // SUB A
            0xED, 0x5F,         // LD A,R
        ];
        cpu.mem.write(0x0000, &prog);

        assert_eq!(9, cpu.step(bus)); assert_eq!(0x01, cpu.reg.get8(A)); assert!(flags(&cpu, PF|CF));
        assert_eq!(4, cpu.step(bus)); assert_eq!(0x00, cpu.reg.get8(A)); assert!(flags(&cpu, ZF|NF));
        assert_eq!(9, cpu.step(bus)); assert_eq!(0x39, cpu.reg.get8(A)); assert!(flags(&cpu, PF));
    }

    #[test]
    fn test_ld_ir_a() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let prog = [
            0x3E, 0x45,     // LD A,0x45
            0xED, 0x47,     // LD I,A
            0xED, 0x4F,     // LD R,A
        ];
        cpu.mem.write(0x0000, &prog);
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x45, cpu.reg.get8(A));
        assert_eq!(9, cpu.step(bus)); assert_eq!(0x45, cpu.reg.i);
        assert_eq!(9, cpu.step(bus)); assert_eq!(0x45, cpu.reg.r);
    }

    #[test]
    fn test_rlc_rl_rrc_rr_r() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let prog = [
            0x3E, 0x01,     // LD A,0x01
            0x06, 0xFF,     // LD B,0xFF
            0x0E, 0x03,     // LD C,0x03
            0x16, 0xFE,     // LD D,0xFE
            0x1E, 0x11,     // LD E,0x11
            0x26, 0x3F,     // LD H,0x3F
            0x2E, 0x70,     // LD L,0x70

            0xCB, 0x0F,     // RRC A
            0xCB, 0x07,     // RLC A
            0xCB, 0x08,     // RRC B
            0xCB, 0x00,     // RLC B
            0xCB, 0x01,     // RLC C
            0xCB, 0x09,     // RRC C
            0xCB, 0x02,     // RLC D
            0xCB, 0x0A,     // RRC D
            0xCB, 0x0B,     // RRC E
            0xCB, 0x03,     // RLC E
            0xCB, 0x04,     // RLC H
            0xCB, 0x0C,     // RCC H
            0xCB, 0x05,     // RLC L
            0xCB, 0x0D,     // RRC L

            0xCB, 0x1F,     // RR A
            0xCB, 0x17,     // RL A
            0xCB, 0x18,     // RR B
            0xCB, 0x10,     // RL B
            0xCB, 0x11,     // RL C
            0xCB, 0x19,     // RR C
            0xCB, 0x12,     // RL D
            0xCB, 0x1A,     // RR D
            0xCB, 0x1B,     // RR E
            0xCB, 0x13,     // RL E
            0xCB, 0x14,     // RL H
            0xCB, 0x1C,     // RR H
            0xCB, 0x15,     // RL L
            0xCB, 0x1D,     // RR L
        ];
        cpu.mem.write(0x0000, &prog);

        // skip loads
        for _ in 0..7 {
            cpu.step(bus);
        }
        assert_eq!(8, cpu.step(bus)); assert_eq!(0x80, cpu.reg.get8(A)); assert!(flags(&cpu, SF|CF));
        assert_eq!(8, cpu.step(bus)); assert_eq!(0x01, cpu.reg.get8(A)); assert!(flags(&cpu, CF));
        assert_eq!(8, cpu.step(bus)); assert_eq!(0xFF, cpu.reg.get8(B)); assert!(flags(&cpu, SF|PF|CF));
        assert_eq!(8, cpu.step(bus)); assert_eq!(0xFF, cpu.reg.get8(B)); assert!(flags(&cpu, SF|PF|CF));
        assert_eq!(8, cpu.step(bus)); assert_eq!(0x06, cpu.reg.get8(C)); assert!(flags(&cpu, PF));
        assert_eq!(8, cpu.step(bus)); assert_eq!(0x03, cpu.reg.get8(C)); assert!(flags(&cpu, PF));
        assert_eq!(8, cpu.step(bus)); assert_eq!(0xFD, cpu.reg.get8(D)); assert!(flags(&cpu, SF|CF));
        assert_eq!(8, cpu.step(bus)); assert_eq!(0xFE, cpu.reg.get8(D)); assert!(flags(&cpu, SF|CF));
        assert_eq!(8, cpu.step(bus)); assert_eq!(0x88, cpu.reg.get8(E)); assert!(flags(&cpu, SF|PF|CF));
        assert_eq!(8, cpu.step(bus)); assert_eq!(0x11, cpu.reg.get8(E)); assert!(flags(&cpu, PF|CF));
        assert_eq!(8, cpu.step(bus)); assert_eq!(0x7E, cpu.reg.get8(H)); assert!(flags(&cpu, PF));
        assert_eq!(8, cpu.step(bus)); assert_eq!(0x3F, cpu.reg.get8(H)); assert!(flags(&cpu, PF));
        assert_eq!(8, cpu.step(bus)); assert_eq!(0xE0, cpu.reg.get8(L)); assert!(flags(&cpu, SF));
        assert_eq!(8, cpu.step(bus)); assert_eq!(0x70, cpu.reg.get8(L)); assert!(flags(&cpu, 0));
        assert_eq!(8, cpu.step(bus)); assert_eq!(0x00, cpu.reg.get8(A)); assert!(flags(&cpu, ZF|PF|CF));
        assert_eq!(8, cpu.step(bus)); assert_eq!(0x01, cpu.reg.get8(A)); assert!(flags(&cpu, 0));
        assert_eq!(8, cpu.step(bus)); assert_eq!(0x7F, cpu.reg.get8(B)); assert!(flags(&cpu, CF));
        assert_eq!(8, cpu.step(bus)); assert_eq!(0xFF, cpu.reg.get8(B)); assert!(flags(&cpu, SF|PF));
        assert_eq!(8, cpu.step(bus)); assert_eq!(0x06, cpu.reg.get8(C)); assert!(flags(&cpu, PF));
        assert_eq!(8, cpu.step(bus)); assert_eq!(0x03, cpu.reg.get8(C)); assert!(flags(&cpu, PF));
        assert_eq!(8, cpu.step(bus)); assert_eq!(0xFC, cpu.reg.get8(D)); assert!(flags(&cpu, SF|PF|CF));
        assert_eq!(8, cpu.step(bus)); assert_eq!(0xFE, cpu.reg.get8(D)); assert!(flags(&cpu, SF));
        assert_eq!(8, cpu.step(bus)); assert_eq!(0x08, cpu.reg.get8(E)); assert!(flags(&cpu, CF));
        assert_eq!(8, cpu.step(bus)); assert_eq!(0x11, cpu.reg.get8(E)); assert!(flags(&cpu, PF));
        assert_eq!(8, cpu.step(bus)); assert_eq!(0x7E, cpu.reg.get8(H)); assert!(flags(&cpu, PF));
        assert_eq!(8, cpu.step(bus)); assert_eq!(0x3F, cpu.reg.get8(H)); assert!(flags(&cpu, PF));
        assert_eq!(8, cpu.step(bus)); assert_eq!(0xE0, cpu.reg.get8(L)); assert!(flags(&cpu, SF));
        assert_eq!(8, cpu.step(bus)); assert_eq!(0x70, cpu.reg.get8(L)); assert!(flags(&cpu, 0));
    }

    #[test]
    fn test_rrc_rlc_rr_rl_ihlixiy() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let data = [ 0x01, 0xFF, 0x11 ];
        cpu.mem.write(0x1000, &data);
        let prog = [
            0x21, 0x00, 0x10,           // LD HL,0x1000
            0xDD, 0x21, 0x00, 0x10,     // LD IX,0x1001
            0xFD, 0x21, 0x03, 0x10,     // LD IY,0x1003
            0xCB, 0x0E,                 // RRC (HL)
            0x7E,                       // LD A,(HL)
            0xCB, 0x06,                 // RLC (HL)
            0x7E,                       // LD A,(HL)
            0xDD, 0xCB, 0x01, 0x0E,     // RRC (IX+1)
            0xDD, 0x7E, 0x01,           // LD A,(IX+1)
            0xDD, 0xCB, 0x01, 0x06,     // RLC (IX+1)
            0xDD, 0x7E, 0x01,           // LD A,(IX+1)
            0xFD, 0xCB, 0xFF, 0x0E,     // RRC (IY-1)
            0xFD, 0x7E, 0xFF,           // LD A,(IY-1)
            0xFD, 0xCB, 0xFF, 0x06,     // RLC (IY-1)
            0xFD, 0x7E, 0xFF,           // LD A,(IY-1)
            0xCB, 0x1E,                 // RR (HL)
            0x7E,                       // LD A,(HL)
            0xCB, 0x16,                 // RL (HL)
            0x7E,                       // LD A,(HL)
            0xDD, 0xCB, 0x01, 0x1E,     // RR (IX+1)
            0xDD, 0x7E, 0x01,           // LD A,(IX+1)
            0xDD, 0xCB, 0x01, 0x16,     // RL (IX+1)
            0xDD, 0x7E, 0x01,           // LD A,(IX+1)
            0xFD, 0xCB, 0xFF, 0x16,     // RL (IY-1)
            0xFD, 0x7E, 0xFF,           // LD A,(IY-1)
            0xFD, 0xCB, 0xFF, 0x1E,     // RR (IY-1)
            0xFD, 0x7E, 0xFF,           // LD A,(IY-1)
        ];
        cpu.mem.write(0x0000, &prog);

        // skip loads
        for _ in 0..3 {
            cpu.step(bus);
        }
        assert_eq!(15, cpu.step(bus)); assert_eq!(0x80, cpu.mem.r8(0x1000)); assert!(flags(&cpu, SF|CF));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x80, cpu.reg.get8(A));
        assert_eq!(15, cpu.step(bus)); assert_eq!(0x01, cpu.mem.r8(0x1000)); assert!(flags(&cpu, CF));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x01, cpu.reg.get8(A));
        assert_eq!(23, cpu.step(bus)); assert_eq!(0xFF, cpu.mem.r8(0x1001)); assert!(flags(&cpu, SF|PF|CF));
        assert_eq!(19, cpu.step(bus)); assert_eq!(0xFF, cpu.reg.get8(A));
        assert_eq!(23, cpu.step(bus)); assert_eq!(0xFF, cpu.mem.r8(0x1001)); assert!(flags(&cpu, SF|PF|CF));
        assert_eq!(19, cpu.step(bus)); assert_eq!(0xFF, cpu.reg.get8(A));
        assert_eq!(23, cpu.step(bus)); assert_eq!(0x88, cpu.mem.r8(0x1002)); assert!(flags(&cpu, SF|PF|CF));
        assert_eq!(19, cpu.step(bus)); assert_eq!(0x88, cpu.reg.get8(A));
        assert_eq!(23, cpu.step(bus)); assert_eq!(0x11, cpu.mem.r8(0x1002)); assert!(flags(&cpu, PF|CF));
        assert_eq!(19, cpu.step(bus)); assert_eq!(0x11, cpu.reg.get8(A));
        assert_eq!(15, cpu.step(bus)); assert_eq!(0x80, cpu.mem.r8(0x1000)); assert!(flags(&cpu, SF|CF));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x80, cpu.reg.get8(A));
        assert_eq!(15, cpu.step(bus)); assert_eq!(0x01, cpu.mem.r8(0x1000)); assert!(flags(&cpu, CF));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x01, cpu.reg.get8(A));
        assert_eq!(23, cpu.step(bus)); assert_eq!(0xFF, cpu.mem.r8(0x1001)); assert!(flags(&cpu, SF|PF|CF));
        assert_eq!(19, cpu.step(bus)); assert_eq!(0xFF, cpu.reg.get8(A));
        assert_eq!(23, cpu.step(bus)); assert_eq!(0xFF, cpu.mem.r8(0x1001)); assert!(flags(&cpu, SF|PF|CF));
        assert_eq!(19, cpu.step(bus)); assert_eq!(0xFF, cpu.reg.get8(A));
        assert_eq!(23, cpu.step(bus)); assert_eq!(0x23, cpu.mem.r8(0x1002)); assert!(flags(&cpu, 0));
        assert_eq!(19, cpu.step(bus)); assert_eq!(0x23, cpu.reg.get8(A));
        assert_eq!(23, cpu.step(bus)); assert_eq!(0x11, cpu.mem.r8(0x1002)); assert!(flags(&cpu, PF|CF));
        assert_eq!(19, cpu.step(bus)); assert_eq!(0x11, cpu.reg.get8(A));
    }

    #[test]
    fn test_sla_r() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let prog = [
            0x3E, 0x01,         // LD A,0x01
            0x06, 0x80,         // LD B,0x80
            0x0E, 0xAA,         // LD C,0xAA
            0x16, 0xFE,         // LD D,0xFE
            0x1E, 0x7F,         // LD E,0x7F
            0x26, 0x11,         // LD H,0x11
            0x2E, 0x00,         // LD L,0x00
            0xCB, 0x27,         // SLA A
            0xCB, 0x20,         // SLA B
            0xCB, 0x21,         // SLA C
            0xCB, 0x22,         // SLA D
            0xCB, 0x23,         // SLA E
            0xCB, 0x24,         // SLA H
            0xCB, 0x25,         // SLA L
        ];
        cpu.mem.write(0x0000, &prog);

        // skip loads
        for _ in 0..7 {
            cpu.step(bus);
        }
        assert_eq!(8, cpu.step(bus)); assert_eq!(0x02, cpu.reg.get8(A)); assert!(flags(&cpu, 0));
        assert_eq!(8, cpu.step(bus)); assert_eq!(0x00, cpu.reg.get8(B)); assert!(flags(&cpu, ZF|PF|CF));
        assert_eq!(8, cpu.step(bus)); assert_eq!(0x54, cpu.reg.get8(C)); assert!(flags(&cpu, CF));
        assert_eq!(8, cpu.step(bus)); assert_eq!(0xFC, cpu.reg.get8(D)); assert!(flags(&cpu, SF|PF|CF));
        assert_eq!(8, cpu.step(bus)); assert_eq!(0xFE, cpu.reg.get8(E)); assert!(flags(&cpu, SF));
        assert_eq!(8, cpu.step(bus)); assert_eq!(0x22, cpu.reg.get8(H)); assert!(flags(&cpu, PF));
        assert_eq!(8, cpu.step(bus)); assert_eq!(0x00, cpu.reg.get8(L)); assert!(flags(&cpu, ZF|PF));
    }

    #[test]
    fn test_sra_r() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let prog = [
            0x3E, 0x01,         // LD A,0x01
            0x06, 0x80,         // LD B,0x80
            0x0E, 0xAA,         // LD C,0xAA
            0x16, 0xFE,         // LD D,0xFE
            0x1E, 0x7F,         // LD E,0x7F
            0x26, 0x11,         // LD H,0x11
            0x2E, 0x00,         // LD L,0x00
            0xCB, 0x2F,         // SRA A
            0xCB, 0x28,         // SRA B
            0xCB, 0x29,         // SRA C
            0xCB, 0x2A,         // SRA D
            0xCB, 0x2B,         // SRA E
            0xCB, 0x2C,         // SRA H
            0xCB, 0x2D,         // SRA L
        ];
        cpu.mem.write(0x0000, &prog);

        // skip loads
        for _ in 0..7 {
            cpu.step(bus);
        }
        assert_eq!(8, cpu.step(bus)); assert_eq!(0x00, cpu.reg.get8(A)); assert!(flags(&cpu, ZF|PF|CF));
        assert_eq!(8, cpu.step(bus)); assert_eq!(0xC0, cpu.reg.get8(B)); assert!(flags(&cpu, SF|PF));
        assert_eq!(8, cpu.step(bus)); assert_eq!(0xD5, cpu.reg.get8(C)); assert!(flags(&cpu, SF));
        assert_eq!(8, cpu.step(bus)); assert_eq!(0xFF, cpu.reg.get8(D)); assert!(flags(&cpu, SF|PF));
        assert_eq!(8, cpu.step(bus)); assert_eq!(0x3F, cpu.reg.get8(E)); assert!(flags(&cpu, PF|CF));
        assert_eq!(8, cpu.step(bus)); assert_eq!(0x08, cpu.reg.get8(H)); assert!(flags(&cpu, CF));
        assert_eq!(8, cpu.step(bus)); assert_eq!(0x00, cpu.reg.get8(L)); assert!(flags(&cpu, ZF|PF));
    }

    #[test]
    fn test_srl_r() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let prog = [
            0x3E, 0x01,         // LD A,0x01
            0x06, 0x80,         // LD B,0x80
            0x0E, 0xAA,         // LD C,0xAA
            0x16, 0xFE,         // LD D,0xFE
            0x1E, 0x7F,         // LD E,0x7F
            0x26, 0x11,         // LD H,0x11
            0x2E, 0x00,         // LD L,0x00
            0xCB, 0x3F,         // SRL A
            0xCB, 0x38,         // SRL B
            0xCB, 0x39,         // SRL C
            0xCB, 0x3A,         // SRL D
            0xCB, 0x3B,         // SRL E
            0xCB, 0x3C,         // SRL H
            0xCB, 0x3D,         // SRL L
        ];
        cpu.mem.write(0x0000, &prog);

        // skip loads
        for _ in 0..7 {
            cpu.step(bus);
        }
        assert_eq!(8, cpu.step(bus)); assert_eq!(0x00, cpu.reg.get8(A)); assert!(flags(&cpu, ZF|PF|CF));
        assert_eq!(8, cpu.step(bus)); assert_eq!(0x40, cpu.reg.get8(B)); assert!(flags(&cpu, 0));
        assert_eq!(8, cpu.step(bus)); assert_eq!(0x55, cpu.reg.get8(C)); assert!(flags(&cpu, PF));
        assert_eq!(8, cpu.step(bus)); assert_eq!(0x7F, cpu.reg.get8(D)); assert!(flags(&cpu, 0));
        assert_eq!(8, cpu.step(bus)); assert_eq!(0x3F, cpu.reg.get8(E)); assert!(flags(&cpu, PF|CF));
        assert_eq!(8, cpu.step(bus)); assert_eq!(0x08, cpu.reg.get8(H)); assert!(flags(&cpu, CF));
        assert_eq!(8, cpu.step(bus)); assert_eq!(0x00, cpu.reg.get8(L)); assert!(flags(&cpu, ZF|PF));
    }

    #[test]
    fn test_sla_ihlixiy() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let data = [ 0x01, 0x80, 0xAA ];
        cpu.mem.write(0x1000, &data);
        let prog = [
            0x21, 0x00, 0x10,           // LD HL,0x1000
            0xDD, 0x21, 0x00, 0x10,     // LD IX,0x1001
            0xFD, 0x21, 0x03, 0x10,     // LD IY,0x1003
            0xCB, 0x26,                 // SLA (HL)
            0x7E,                       // LD A,(HL)
            0xDD, 0xCB, 0x01, 0x26,     // SLA (IX+1)
            0xDD, 0x7E, 0x01,           // LD A,(IX+1)
            0xFD, 0xCB, 0xFF, 0x26,     // SLA (IY-1)
            0xFD, 0x7E, 0xFF,           // LD A,(IY-1)
        ];
        cpu.mem.write(0x0000, &prog);

        // skip loads
        for _ in 0..3 {
            cpu.step(bus);
        }
        assert_eq!(15, cpu.step(bus)); assert_eq!(0x02, cpu.mem.r8(0x1000)); assert!(flags(&cpu, 0));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x02, cpu.reg.get8(A));
        assert_eq!(23, cpu.step(bus)); assert_eq!(0x00, cpu.mem.r8(0x1001)); assert!(flags(&cpu, ZF|PF|CF));
        assert_eq!(19, cpu.step(bus)); assert_eq!(0x00, cpu.reg.get8(A));
        assert_eq!(23, cpu.step(bus)); assert_eq!(0x54, cpu.mem.r8(0x1002)); assert!(flags(&cpu, CF));
        assert_eq!(19, cpu.step(bus)); assert_eq!(0x54, cpu.reg.get8(A));
    }

    #[test]
    fn test_sra_ihlixiy() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let data = [ 0x01, 0x80, 0xAA ];
        cpu.mem.write(0x1000, &data);
        let prog = [
            0x21, 0x00, 0x10,           // LD HL,0x1000
            0xDD, 0x21, 0x00, 0x10,     // LD IX,0x1001
            0xFD, 0x21, 0x03, 0x10,     // LD IY,0x1003
            0xCB, 0x2E,                 // SRA (HL)
            0x7E,                       // LD A,(HL)
            0xDD, 0xCB, 0x01, 0x2E,     // SRA (IX+1)
            0xDD, 0x7E, 0x01,           // LD A,(IX+1)
            0xFD, 0xCB, 0xFF, 0x2E,     // SRA (IY-1)
            0xFD, 0x7E, 0xFF,           // LD A,(IY-1)
        ];
        cpu.mem.write(0x000, &prog);

        // skip loads
        for _ in 0..3 {
            cpu.step(bus);
        }
        assert_eq!(15, cpu.step(bus)); assert_eq!(0x00, cpu.mem.r8(0x1000)); assert!(flags(&cpu, ZF|PF|CF));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x00, cpu.reg.get8(A));
        assert_eq!(23, cpu.step(bus)); assert_eq!(0xC0, cpu.mem.r8(0x1001)); assert!(flags(&cpu, SF|PF));
        assert_eq!(19, cpu.step(bus)); assert_eq!(0xC0, cpu.reg.get8(A));
        assert_eq!(23, cpu.step(bus)); assert_eq!(0xD5, cpu.mem.r8(0x1002)); assert!(flags(&cpu, SF));
        assert_eq!(19, cpu.step(bus)); assert_eq!(0xD5, cpu.reg.get8(A));
    }

    #[test]
    fn test_srl_ihlixiy() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let data = [ 0x01, 0x80, 0xAA ];
        cpu.mem.write(0x1000, &data);
        let prog = [
            0x21, 0x00, 0x10,           // LD HL,0x1000
            0xDD, 0x21, 0x00, 0x10,     // LD IX,0x1001
            0xFD, 0x21, 0x03, 0x10,     // LD IY,0x1003
            0xCB, 0x3E,                 // SRL (HL)
            0x7E,                       // LD A,(HL)
            0xDD, 0xCB, 0x01, 0x3E,     // SRL (IX+1)
            0xDD, 0x7E, 0x01,           // LD A,(IX+1)
            0xFD, 0xCB, 0xFF, 0x3E,     // SRL (IY-1)
            0xFD, 0x7E, 0xFF,           // LD A,(IY-1)
        ];
        cpu.mem.write(0x000, &prog);

        // skip loads
        for _ in 0..3 {
            cpu.step(bus);
        }
        assert_eq!(15, cpu.step(bus)); assert_eq!(0x00, cpu.mem.r8(0x1000)); assert!(flags(&cpu, ZF|PF|CF));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x00, cpu.reg.get8(A));
        assert_eq!(23, cpu.step(bus)); assert_eq!(0x40, cpu.mem.r8(0x1001)); assert!(flags(&cpu, 0));
        assert_eq!(19, cpu.step(bus)); assert_eq!(0x40, cpu.reg.get8(A));
        assert_eq!(23, cpu.step(bus)); assert_eq!(0x55, cpu.mem.r8(0x1002)); assert!(flags(&cpu, PF));
        assert_eq!(19, cpu.step(bus)); assert_eq!(0x55, cpu.reg.get8(A));
    }

    #[test]
    fn test_rld_rrd() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let prog = [
            0x3E, 0x12,         // LD A,0x12
            0x21, 0x00, 0x10,   // LD HL,0x1000
            0x36, 0x34,         // LD (HL),0x34
            0xED, 0x67,         // RRD
            0xED, 0x6F,         // RLD
            0x7E,               // LD A,(HL)
            0x3E, 0xFE,         // LD A,0xFE
            0x36, 0x00,         // LD (HL),0x00
            0xED, 0x6F,         // RLD
            0xED, 0x67,         // RRD
            0x7E,               // LD A,(HL)
            0x3E, 0x01,         // LD A,0x01
            0x36, 0x00,         // LD (HL),0x00
            0xED, 0x6F,         // RLD
            0xED, 0x67,         // RRD
            0x7E
        ];
        cpu.mem.write(0x0000, &prog);

        assert_eq!(7, cpu.step(bus)); assert_eq!(0x12, cpu.reg.get8(A));
        assert_eq!(10, cpu.step(bus)); assert_eq!(0x1000, cpu.reg.get16(HL));
        assert_eq!(10, cpu.step(bus)); assert_eq!(0x34, cpu.mem.r8(0x1000));
        assert_eq!(18, cpu.step(bus)); assert_eq!(0x14, cpu.reg.get8(A)); assert_eq!(0x23, cpu.mem.r8(0x1000));
        assert_eq!(18, cpu.step(bus)); assert_eq!(0x12, cpu.reg.get8(A)); assert_eq!(0x34, cpu.mem.r8(0x1000));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x34, cpu.reg.get8(A));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0xFE, cpu.reg.get8(A));
        assert_eq!(10, cpu.step(bus)); assert_eq!(0x00, cpu.mem.r8(0x1000));
        assert_eq!(18, cpu.step(bus)); assert_eq!(0xF0, cpu.reg.get8(A)); assert_eq!(0x0E, cpu.mem.r8(0x1000)); assert!(flags(&cpu, SF|PF));
        assert_eq!(18, cpu.step(bus)); assert_eq!(0xFE, cpu.reg.get8(A)); assert_eq!(0x00, cpu.mem.r8(0x1000)); assert!(flags(&cpu, SF));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x00, cpu.reg.get8(A));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x01, cpu.reg.get8(A));
        assert_eq!(10, cpu.step(bus)); assert_eq!(0x00, cpu.mem.r8(0x1000));
        let f = cpu.reg.get8(F) | CF;
        cpu.reg.set8(F, f);
        assert_eq!(18, cpu.step(bus)); assert_eq!(0x00, cpu.reg.get8(A)); assert_eq!(0x01, cpu.mem.r8(0x1000)); assert!(flags(&cpu, ZF|PF|CF));
        assert_eq!(18, cpu.step(bus)); assert_eq!(0x01, cpu.reg.get8(A)); assert_eq!(0x00, cpu.mem.r8(0x1000)); assert!(flags(&cpu, CF));
        assert_eq!(7, cpu.step(bus)); assert_eq!(0x00, cpu.reg.get8(A));
    }

    #[test]
    fn test_in() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let prog = [
            0x3E, 0x01,         // LD A,0x01
            0xDB, 0x03,         // IN A,(0x03)
            0xDB, 0x04,         // IN A,(0x04)
            0x01, 0x02, 0x02,   // LD BC,0x0202
            0xED, 0x78,         // IN A,(C)
            0x01, 0xFF, 0x05,   // LD BC,0x05FF
            0xED, 0x50,         // IN D,(C)
            0x01, 0x05, 0x05,   // LD BC,0x0505
            0xED, 0x58,         // IN E,(C)
            0x01, 0x06, 0x01,   // LD BC,0x0106
            0xED, 0x60,         // IN H,(C)
            0x01, 0x00, 0x10,   // LD BC,0x0000
            0xED, 0x68,         // IN L,(C)
            0xED, 0x40,         // IN B,(C)
            0xED, 0x48,         // IN C,(B)
        ];
        cpu.mem.write(0x0000, &prog);
        cpu.reg.set8(F, HF|CF);

        assert_eq!(7, cpu.step(bus)); assert_eq!(0x01, cpu.reg.get8(A)); assert!(flags(&cpu, HF|CF));
        assert_eq!(11, cpu.step(bus)); assert_eq!(0x06, cpu.reg.get8(A)); assert!(flags(&cpu, HF|CF));
        assert_eq!(11, cpu.step(bus)); assert_eq!(0x08, cpu.reg.get8(A)); assert!(flags(&cpu, HF|CF));
        assert_eq!(10, cpu.step(bus)); assert_eq!(0x0202, cpu.reg.get16(BC));
        assert_eq!(12, cpu.step(bus)); assert_eq!(0x04, cpu.reg.get8(A)); assert!(flags(&cpu, CF));
        assert_eq!(10, cpu.step(bus)); assert_eq!(0x05FF, cpu.reg.get16(BC));
        assert_eq!(12, cpu.step(bus)); assert_eq!(0xFE, cpu.reg.get8(D)); assert!(flags(&cpu, SF|CF));
        assert_eq!(10, cpu.step(bus)); assert_eq!(0x0505, cpu.reg.get16(BC));
        assert_eq!(12, cpu.step(bus)); assert_eq!(0x0A, cpu.reg.get8(E)); assert!(flags(&cpu, PF|CF));
        assert_eq!(10, cpu.step(bus)); assert_eq!(0x0106, cpu.reg.get16(BC));
        assert_eq!(12, cpu.step(bus)); assert_eq!(0x0C, cpu.reg.get8(H)); assert!(flags(&cpu, PF|CF));
        assert_eq!(10, cpu.step(bus)); assert_eq!(0x1000, cpu.reg.get16(BC));
        assert_eq!(12, cpu.step(bus)); assert_eq!(0x00, cpu.reg.get8(L)); assert!(flags(&cpu, ZF|PF|CF));
        assert_eq!(12, cpu.step(bus)); assert_eq!(0x00, cpu.reg.get8(B)); assert!(flags(&cpu, ZF|PF|CF));
        assert_eq!(12, cpu.step(bus)); assert_eq!(0x00, cpu.reg.get8(C)); assert!(flags(&cpu, ZF|PF|CF));
    }

    #[test]
    fn test_out() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let prog = [
            0x3E, 0x01,         // LD A,0x01
            0xD3, 0x01,         // OUT (0x01),A
            0xD3, 0x02,         // OUT (0x02),A
            0x01, 0x34, 0x12,   // LD BC,0x1234
            0x11, 0x78, 0x56,   // LD DE,0x5678
            0x21, 0xCD, 0xAB,   // LD HL,0xABCD
            0xED, 0x79,         // OUT (C),A
            0xED, 0x41,         // OUT (C),B
            0xED, 0x49,         // OUT (C),C
            0xED, 0x51,         // OUT (C),D
            0xED, 0x59,         // OUT (C),E
            0xED, 0x61,         // OUT (C),H
            0xED, 0x69,         // OUT (C),L
        ];
        cpu.mem.write(0x0000, &prog);

        assert_eq!(7, cpu.step(bus)); assert_eq!(0x01, cpu.reg.get8(A));
        assert_eq!(11, cpu.step(bus)); assert_eq!(0x0101, bus.port.get()); assert_eq!(0x01, bus.val.get());
        assert_eq!(11, cpu.step(bus)); assert_eq!(0x0102, bus.port.get()); assert_eq!(0x01, bus.val.get());
        assert_eq!(10, cpu.step(bus)); assert_eq!(0x1234, cpu.reg.get16(BC));
        assert_eq!(10, cpu.step(bus)); assert_eq!(0x5678, cpu.reg.get16(DE));
        assert_eq!(10, cpu.step(bus)); assert_eq!(0xABCD, cpu.reg.get16(HL));
        assert_eq!(12, cpu.step(bus)); assert_eq!(0x1234, bus.port.get()); assert_eq!(0x01, bus.val.get());
        assert_eq!(12, cpu.step(bus)); assert_eq!(0x1234, bus.port.get()); assert_eq!(0x12, bus.val.get());
        assert_eq!(12, cpu.step(bus)); assert_eq!(0x1234, bus.port.get()); assert_eq!(0x34, bus.val.get());
        assert_eq!(12, cpu.step(bus)); assert_eq!(0x1234, bus.port.get()); assert_eq!(0x56, bus.val.get());
        assert_eq!(12, cpu.step(bus)); assert_eq!(0x1234, bus.port.get()); assert_eq!(0x78, bus.val.get());
        assert_eq!(12, cpu.step(bus)); assert_eq!(0x1234, bus.port.get()); assert_eq!(0xAB, bus.val.get());
        assert_eq!(12, cpu.step(bus)); assert_eq!(0x1234, bus.port.get()); assert_eq!(0xCD, bus.val.get());
    }

    #[test]
    fn test_inir_indr() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let prog = [
            0x21, 0x00, 0x10,       // LD HL,0x1000
            0x01, 0x02, 0x03,       // LD BC,0x0302
            0xED, 0xB2,             // INIR
            0x01, 0x03, 0x03,       // LD BC,0x0303
            0xED, 0xBA              // INDR
        ];
        cpu.mem.write(0x0000, &prog);

        assert_eq!(10, cpu.step(bus)); assert_eq!(0x1000, cpu.reg.get16(HL));
        assert_eq!(10, cpu.step(bus)); assert_eq!(0x0302, cpu.reg.get16(BC));

        assert_eq!(21, cpu.step(bus));
        assert_eq!(0x1001, cpu.reg.get16(HL));
        assert_eq!(0x0202, cpu.reg.get16(BC));
        assert_eq!(0x04, cpu.mem.r8(0x1000));
        assert_eq!((cpu.reg.get8(F) & ZF), 0);
        assert_eq!(21, cpu.step(bus));
        assert_eq!(0x1002, cpu.reg.get16(HL));
        assert_eq!(0x0102, cpu.reg.get16(BC));
        assert_eq!(0x04, cpu.mem.r8(0x1001));
        assert_eq!((cpu.reg.get8(F) & ZF), 0);
        assert_eq!(16, cpu.step(bus));
        assert_eq!(0x1003, cpu.reg.get16(HL));
        assert_eq!(0x0002, cpu.reg.get16(BC));
        assert_eq!(0x04, cpu.mem.r8(0x1002));
        assert!((cpu.reg.get8(F) & ZF) != 0);

        assert_eq!(10, cpu.step(bus)); assert_eq!(0x0303, cpu.reg.get16(BC));
        assert_eq!(21, cpu.step(bus));
        assert_eq!(0x1002, cpu.reg.get16(HL));
        assert_eq!(0x0203, cpu.reg.get16(BC));
        assert_eq!(0x06, cpu.mem.r8(0x1003));
        assert_eq!((cpu.reg.get8(F) & ZF), 0);
        assert_eq!(21, cpu.step(bus));
        assert_eq!(0x1001, cpu.reg.get16(HL));
        assert_eq!(0x0103, cpu.reg.get16(BC));
        assert_eq!(0x06, cpu.mem.r8(0x1002));
        assert_eq!((cpu.reg.get8(F) & ZF), 0);
        assert_eq!(16, cpu.step(bus));
        assert_eq!(0x1000, cpu.reg.get16(HL));
        assert_eq!(0x0003, cpu.reg.get16(BC));
        assert_eq!(0x06, cpu.mem.r8(0x1001));
        assert!((cpu.reg.get8(F) & ZF) != 0);
    }

    #[test]
    fn test_otir_otdr() {
        let mut cpu = rz80::CPU::new_64k();
        let bus = &TestBus::new();
        let data = [ 0x01, 0x02, 0x03, 0x04 ];
        cpu.mem.write(0x1000, &data);
        let prog = [
            0x21, 0x00, 0x10,       // LD HL,0x1000
            0x01, 0x02, 0x03,       // LD BC,0x0302
            0xED, 0xB3,             // OTIR
            0x01, 0x03, 0x03,       // LD BC,0x0303
            0xED, 0xBB,             // OTDR
        ];
        cpu.mem.write(0x0000, &prog);

        assert_eq!(10, cpu.step(bus)); assert_eq!(0x1000, cpu.reg.get16(HL));
        assert_eq!(10, cpu.step(bus)); assert_eq!(0x0302, cpu.reg.get16(BC));
        assert_eq!(21, cpu.step(bus));
        assert_eq!(0x1001, cpu.reg.get16(HL));
        assert_eq!(0x0202, cpu.reg.get16(BC));
        assert_eq!(0x0202, bus.port.get()); assert_eq!(0x01, bus.val.get());
        assert_eq!((cpu.reg.get8(F) & ZF), 0);
        assert_eq!(21, cpu.step(bus));
        assert_eq!(0x1002, cpu.reg.get16(HL));
        assert_eq!(0x0102, cpu.reg.get16(BC));
        assert_eq!(0x0102, bus.port.get()); assert_eq!(0x02, bus.val.get());
        assert_eq!((cpu.reg.get8(F) & ZF), 0);
        assert_eq!(16, cpu.step(bus));
        assert_eq!(0x1003, cpu.reg.get16(HL));
        assert_eq!(0x0002, cpu.reg.get16(BC));
        assert_eq!(0x0002, bus.port.get()); assert_eq!(0x03, bus.val.get());
        assert!((cpu.reg.get8(F) & ZF) != 0);
        assert_eq!(10, cpu.step(bus)); assert_eq!(0x0303, cpu.reg.get16(BC));
        assert_eq!(21, cpu.step(bus));
        assert_eq!(0x1002, cpu.reg.get16(HL));
        assert_eq!(0x0203, cpu.reg.get16(BC));
        assert_eq!(0x0203, bus.port.get()); assert_eq!(0x04, bus.val.get());
        assert_eq!((cpu.reg.get8(F) & ZF), 0);
        assert_eq!(21, cpu.step(bus));
        assert_eq!(0x1001, cpu.reg.get16(HL));
        assert_eq!(0x0103, cpu.reg.get16(BC));
        assert_eq!(0x0103, bus.port.get()); assert_eq!(0x03, bus.val.get());
        assert_eq!((cpu.reg.get8(F) & ZF), 0);
        assert_eq!(16, cpu.step(bus));
        assert_eq!(0x1000, cpu.reg.get16(HL));
        assert_eq!(0x0003, cpu.reg.get16(BC));
        assert_eq!(0x0003, bus.port.get()); assert_eq!(0x02, bus.val.get());
        assert!((cpu.reg.get8(F) & ZF) != 0);
    }
}
