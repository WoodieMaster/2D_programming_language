# Overview

- The code is written in 2D Grid of unicode characters
- The code is executed by any number of instruction pointers
- Every instruction pointer contains a 
- The program starts with one instruction pointer who starts at the top-left of the file and moves downwards by default.
- If no instruction pointer is left, the program ends
- Every update cycle the instruction pointers are updated in order starting with the earliest created
- The program uses a stack to keep track of data

# Instruction Pointer

- Consists of a position in the code grid and a movement vector
- On the update the instruction pointer reads the character at its position and executes the corresponding instruction. After that it moves defined by its movement vector.
- If the instruction pointer moves outside the code grid, its position wraps around to the opposite side

# Instructions
| Character | Definition                                                                                                                                       |
|-----------|--------------------------------------------------------------------------------------------------------------------------------------------------|
| `>`       | set the movement vector of the instruction pointer to one character to the right                                                                 |
| `<`       | set the movement vector of the instruction pointer to one character to the left                                                                  |
| `^`       | set the movement vector of the instruction pointer to one character upwards                                                                      |
| `v`       | set the movement vector of the instruction pointer to one character downwards                                                                    |
| ` `       | noop                                                                                                                                             |
| `#`       | remove the instruction pointer                                                                                                                   |
| `\|`      | create an additional instruction pointer, which moves one character upwards. Make this instruction pointer move one character downwards          |
| `_`       | create an additional instruction pointer, which moves one character to the right. Set the current instruction pointer to move one character left |
| `"`       | toggle [string mode](#string-mode)                                                                                                               |
| `.`       | pop top value of the stack and print it to stdout                                                                                                |

## String Mode
When an instruction pointer has string mode enabled, every character it visits is put onto the stack instead of executed.