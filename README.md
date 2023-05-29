## A very simple 8-bit CPU simulator

A very simple representation of an 8-bit CPU (I guess you could also say it's a virtual machine). It has two registers, `register_a` and `register_b`, and the following instructions:

| Opcode | Description |
| -------| ------------|
| 0 | No operation |
| 1 | Set value in register A |
| 2 | Set value in register B |
| 3 | Set memory address to register A |
| 4 | Retrieve position stored in register A and store the item at that position into register A |
| 5 | Add values in registers A and B, storing the result in register A |
| 6 | Subtract value in register B from register A, storing the result in register A |
| 7 | Jump to a position in the memory |
| 8 | Jump to a position in the memory if the value in register A is zero |
| 9 | Print the value in register A |
| 10 | Print the character corresponding to the ASCII value in register A |
| 11 | Halt and print a new line |
| 12 | Increment value in register B by 1 |

### Example

The main function in the `main.rs` file includes an example program that prints out the string "Hello, World!".

### Future plans

It would be cool to expand the capabilities of this CPU later. Here are some things that I have in mind:

- Additional instructions such as multiplication, division, logical operations, shift operations, etc.
- A flags register that gets updated after each operation. These could include zero flag, carry flag, sign flag, etc.
- Error handling for invalid opcodes, memory out of bounds access, underflow/overflow of values, etc.
- A simple assembler that can take in a program written in an assembly-like language and output the corresponding `Vec<u8>` that can be executed by the CPU.
- A GUI that visually represents the registers, memory, and the operations being performed, perhaps even allowing you to step through the execution one operation at a time.
- Unit tests.
