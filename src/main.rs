struct CPU {
    register_a: u8,
    register_b: u8,
}

impl CPU {
    fn set_a(&mut self, value: u8) {
        self.register_a = value;
    }

    fn set_b(&mut self, value: u8) {
        self.register_b = value;
    }

    fn set_mem_a(&mut self, memory: &Vec<u8>, position: u8) {
        self.register_a = memory[position as usize];
    }

    fn add(&mut self) {
        self.register_a += self.register_b;
    }

    fn sub(&mut self) {
        self.register_a -= self.register_b;
    }

    fn jump(&mut self, position: u8) -> usize {
        position as usize
    }

    fn jump_if_zero(&mut self, position: u8, current_position: usize) -> usize {
        if self.register_a == 0 {
            return position as usize - 1;
        }
        current_position + 1
    }

    fn print(&mut self) {
        println!("{}", self.register_a);
    }

    fn print_char(&mut self) {
        print!("{}", char::from(self.register_a));
    }

    fn exec_mem(&mut self, memory: Vec<u8>) {
        let mut i = 0;
        while i < memory.len() {
            match memory[i] {
                0 => (),
                1 => {
                    self.set_a(memory[i + 1]);
                    i += 1;
                },
                2 => {
                    self.set_b(memory[i + 1]);
                    i += 1;
                },
                3 => {
                    self.set_mem_a(&memory, memory[i + 1]);
                    i += 1;
                }
                4 => {
                    self.set_mem_a(&memory, self.register_a);
                },
                5 => self.add(),
                6 => self.sub(),
                7 => i = self.jump(memory[i + 1]) - 1,
                8 => i = self.jump_if_zero(memory[i + 1], i),
                9 => self.print(),
                10 => self.print_char(),
                11 => return println!(""),
                12 => self.register_b += 1,
                _ => panic!("{}", memory[i]),
            }
            i += 1;
        }
    }
}

fn main() {
    let example_memory: Vec<u8> = vec![
        1, 16,  // Put 16 in register_a
        2, 1,   // Put 1 in register_b
        4,      // Retrieve position stored in register_a and store the item at that position into register_a
        8, 15,  // Jump to position 15 in memory if register_a is 0
        10,     // Print character in register_a
        3, 1,   // Put item at position 1 in memory into register_a
        5,      // Add item in register_b to register_a and store result in register_a
        12,     // Increment item in register_b by 1
        4,      // Retrieve position stored in register_a and store the item at that position into register_a
        7, 5,   // Jump to position 5 in memory
        11,     // Halt after jump if register_a is 0
        72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100, 33,   // "Hello World!"
        0,      // End of string
    ];

    let mut cpu = CPU {
        register_a: 0,
        register_b: 0,
    };

    cpu.exec_mem(example_memory);
}
