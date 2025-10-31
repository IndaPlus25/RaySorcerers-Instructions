# DD1337 Week 8
Author: Viola Söderlund  
Modified by: Isak Larsson & Benjamin Widman

## Getting started with RISC-V

### Install RARS

To run RISC-V assembly code you need a simulator, since you probably aren't running on a RISC-V computer.

1) Install Java, if you haven't done it already. Make sure that you have at least **Java 8**. 
2) [Download RARS](https://github.com/TheThirdOne/rars) as a `.jar` file. This is the same application as you will use during *IS1500 Datorteknik och komponenter* next year.
3) Open RARS by running the `.jar` file, i.e. double click it.

### Prepare for your assignment

1) Create a repository named `<KTH_ID>-assembly`.
2) Clone your newly created repository.

## Assignments

This week you're going to complete two subassignments. The first assignment is to translate a program written in C to RISC-V assembly, and the second one is to write a specified application in RISC-V assembly.

See `./examples` for RISC-V code examples.

### Assignment 1 - High level => Low level

Learn what it means to be a compiler by translating C to RISC-V assembly instructions. 

Your task is to write a file `./high-to-low/multiplication.asm`, which contains the same algorithms and logic as in `./high-to-low/multiplication.c`. To clarify: **you may only use the `add` instruction, not `mul` (or similar).**

### Assignment 2 - Sieve of Eratosthenes

Write an application, which takes one integer `1 < n <= 1000` as input and prints all prime numbers up to that integer. The prime number search algorithm **must** be an implementation of [Sieve of Eratosthenes](https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes). 

For help with code setup, begin by reviewing the contents of `./sieve/template.asm`.

## Questions

**Constant declarations:**  
With reference to `./sieve/template.asm`,

Know the answer to the following questions:
- Why do array declarations in fast languages, like Rust and C, require the given length to be of constant value?
- A single programming language can define strings in many different ways. For example, Rust has string literals and a String structure. What are the advantages of having both low- and high-level representations of strings? 

**CPU optimisations:**

Observe the following pieces of code:
```riscv
main:
    #...

    # exit program
    li a7, 10                      # set system/environment call code to "terminate program"
    ecall                         # terminate program

    #...
```
```riscv
main:
    #...

    # exit program
    j  exit_program                 # jump to exit_program
    nop

#...

exit_program:
    # EOF (End Of File)
```

Know the answer to the following question:
- Which method of program termination is to prefer and why?

## Reading Material

The [RISC-V Reference sheet](riscv-instruction-sheet.pdf) is *extremely* useful. Some terminology explained:

- `rd` means *registry destination* (registry written to)
- `rs1` and `rs2` means *registry source* (registry read from)
- `imm` means *immediate* (number value)
- `label` is a name for a part of the program, like `main` used for a subroutine inside `.text` or `string` used for a piece of data inside `.data`.
- `PC` is the *program counter*, which is the instruction address to be executed.

For syscall/ecall usage, check out the [RARS environment calls](https://github.com/TheThirdOne/rars/wiki/Environment-Calls).

All of this, like `li a0,10` and `la t1,string`, can also be found inside the help pop up in RARS.

### Pseudo instructions

RISC-V has some pseudo instructions, which are instructions that assemble to other real instructions. Some of these which you might find helpful:

- `li rd, imm`: load immediate, rd = imm
- `la rd, label`: load address, rd = label
- `j label`: jump to label, PC = label
- `jr rd`: jump to instruction address in rd, PC = rd
