##################################################################
#
#   RISC-V assembly code example
#   - Hello World as a global function (routine) with subroutine
#
#   Author: Tobias Hansson <tohans@kth.se>, Viola Söderlund <violaso@kth.se>
#
#   Created: 2020-10-23
#   Translated from MIPS to RISC-V 2024-10-31 by Benjamin Widman <bwidman@kth.se>
#
#   See: RISC-V Syscall Sheet (https://github.com/TheThirdOne/rars/wiki/Environment-Calls)
#   See: RISC-V Instruction Sheet (https://github.com/TheThirdOne/rars/wiki/Supported-Instructions)
#
##################################################################


# This example is a version which could be called as a function.
# For example, this could be called from C if linked with the C program, since hello_world is declared as global.
# NOTE: this crashes on it's own since it returns to a non-existing address.


##
# Push value to application stack.
# PARAM: Registry with value.
##
.macro	PUSH (%reg)
	addi	sp,sp,-4              # decrement stack pointer (stack builds "downwards" in memory)
	sw	%reg,0(sp)             # save value to stack
.end_macro

##
# Pop value from application stack.
# PARAM: Registry which to save value to.
##
.macro	POP (%reg)
	lw	%reg,0(sp)             # load value from stack to given registry
	addi	sp,sp,4               # increment stack pointer (stack builds "downwards" in memory)
.end_macro

### Data Declaration Section ###

.data
HW:     .asciz "Hello World\n"     # define label HW as a constant string "Hello World\n"

### Executable Code Section ###

.globl hello_world                  # define label main as externally accessable 
.text
 
##
#   Hello World routine
#   - print "Hello World\n" to output stream
##
hello_world:                        # function main starts here
    # manage saved regitries
    add		s1, ra, zero            # save return address
    PUSH(s1)                       # save parent routine's saved value to stack

    # call print_hello_world subroutine
    jal		ra, print_hello_world	   # jump to print_hello_world and save position to ra

    # manage saved regitries
    add		ra, s1, zero            # restore return adress
    POP(s1)                        # restore parent routine's saved value from stack

    # return
    jr      ra                     # return to where hello_world was called from

print_hello_world:
    # print HW
    li      a7, 4                  # set syscall code "print string"
    la      a0, HW                 # load address of string HW into syscall argument registry
    ecall                          # print "Hello World\n" to standard output stream

    # return
    jr      ra                     # return to parent routine

##################################################################
#
#   NOTE:
#       Return address is saved so that subroutines within the 
#       hello_world routine can be called.
#
#       A word has the same size as a memory address, which for MIPS
#       is 4 bytes. Thereby the stack pointer jumps values by 4 bytes.
#
##################################################################
