struct Cpu {
    registers: [u16; 16],
    memory: [u16; 2048],
    position_in_memory: usize,
    stack: [usize; 16],
    stack_pointer: usize,
}

impl Cpu {

    fn call(&mut self, addr: usize) {
        if self.stack_pointer >= self.stack.len() {
            panic!("Stack overflow!");
        }
        self.stack_pointer += 1;
        self.stack[self.stack_pointer] = self.position_in_memory;
        self.position_in_memory = addr;
    }

    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack underflow!");
        }
        let addr = self.stack[self.stack_pointer];
        self.stack_pointer -= 1;
        self.position_in_memory = addr;
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let arg0 = self.registers[x as usize];
        let arg1 = self.registers[y as usize];
        let (val, overflow) = arg0.overflowing_add(arg1);

        self.registers[x as usize] = val;

        if overflow {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }

    fn run(&mut self) {
        loop {
            let current_operation = self.memory[self.position_in_memory];
            self.position_in_memory += 1;
            let c = ((current_operation & 0xF000) >> 12) as u8;
            let x = ((current_operation & 0x0F00) >> 8) as u8;
            let y = ((current_operation & 0x00F0) >> 4) as u8;
            let d = ((current_operation & 0x000F) >> 0) as u8;

            match (c, x, y, d) {
                (0xF, 0xF, 0xF, 0xF) => (),
                (0, 0, 0, 0,) => { break; }
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                (0x2, _, _, _) => {
                    let addr = (current_operation & 0x0FFF) as usize;
                    self.call(addr);
                }
                (0x0, 0x0, 0xE, 0xE) => self.ret(),
                _ => todo!("operand_count: {}, op: {}.", c, d),
            }
        }
    }
}

fn main() {
    let mut cpu = Cpu {
        registers: [0; 16],
        memory: [0; 2048],
        position_in_memory: 0,
        stack: [0; 16],
        stack_pointer: 0,
    };

    let add_twice: &[u16] = &[
        0xFFFF,
        0x8014,
        0x8014,
        0x00EE,
    ];

    cpu.registers[0] = 1;
    cpu.registers[1] = 10;
    cpu.registers[2] = 2;
    cpu.registers[3] = 3;

    cpu.memory[0] = 0x8014;
    cpu.memory[1] = 0x8024;
    cpu.memory[2] = 0x8034;
    cpu.memory[3] = 0x2100;
    cpu.memory[4] = 0x2100;

    cpu.memory[0x100..0x104].copy_from_slice(add_twice);

    cpu.run();

    print!("Sum is {}", cpu.registers[0]);
}