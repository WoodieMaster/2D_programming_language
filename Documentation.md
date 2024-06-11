# Overview

- The code is written in 2D Grid of unicode characters
- The code is executed by any number of [instruction pointers](#instruction-pointer)
- Every instruction pointer contains a 
- The program starts with one instruction pointer who starts at the top-left of the file and moves downwards by default.
- Every update cycle the instruction pointers are updated in order starting with the earliest created
- If no instruction pointer is left, the program ends
- The program uses a [stack](#program-stack) to keep track of data

# Instruction Pointer

- Consists of a position in the code grid and a movement vector
- On the update the instruction pointer reads the character at its position and executes the corresponding [instruction](#instructions). After that it moves defined by its movement vector.
- If the instruction pointer moves outside the code grid, its position wraps around to the opposite side
- Each instruction pointer has a set of flags used to mark certain events that happened in the last instruction -> reset for every instruction

# Instructions
| Character | Definition                                                                                                                                       |
|:---------:|--------------------------------------------------------------------------------------------------------------------------------------------------|
|    ` `    | noop                                                                                                                                             |
|    `;`    | remove the instruction pointer                                                                                                                   |
|    `>`    | set the movement vector of the instruction pointer to one character to the right                                                                 |
|    `<`    | set the movement vector of the instruction pointer to one character to the left                                                                  |
|    `^`    | set the movement vector of the instruction pointer to one character upwards                                                                      |
|    `v`    | set the movement vector of the instruction pointer to one character downwards                                                                    |
|   `\|`    | create an additional instruction pointer, which moves one character upwards. Make this instruction pointer move one character downwards          |
|    `_`    | create an additional instruction pointer, which moves one character to the right. Set the current instruction pointer to move one character left |
|    `"`    | toggle [string mode](#string-mode)                                                                                                               |
|    `.`    | pop top value of the stack and print it to stdout                                                                                                |
|   `\ `    | if in string mode escapes the next character (execute instruction or print special character)                                                    |
|    `?`    | skips next instruction if the specified [flag](#flags) is not set.                                                                               |

## Modes
### String Mode
When an instruction pointer has string mode enabled, every character it visits is put onto the stack instead of executed.

## Flags
- each instruction pointer stores its own set of flags
- every instruction overrides all the flags -> every instruction can only read the flags from the previous instruction
- when an instruction requires flag specification, the instruction pointer moves forward and uses that require as access character to define which flag to use -> if the specified access character is invalid it will act like the flag is unset

|    Flag     | Access Character | Definition                                                                  |
|:-----------:|:----------------:|-----------------------------------------------------------------------------|
| empty_stack |       `_`        | is set if an instruction could not pop from the stack, because it was empty |

# Program Stack
A modifiable list of i32 numbers used to store data for the program

## Operations
| Operation | Definition                                     |
|:---------:|------------------------------------------------|
|  `push`   | append value to the top of stack               |
|   `pop`   | remove value from the top of stack             |
|  `peak`   | get top value of the stack without removing it |
| `reverse` | reverse the stack                              |
