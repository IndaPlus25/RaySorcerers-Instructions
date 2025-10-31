##################################################################
#
#   RISC-V assembly code example
#   - Hello World as application 
#
#   Author: Tobias Hansson <tohans@kth.se>
#
#   Created: 2020-10-23
#   Translated from MIPS to RISC-V 2024-10-31 by Benjamin Widman <bwidman@kth.se>
#
#   See: RISC-V Syscall Sheet (https://github.com/TheThirdOne/rars/wiki/Environment-Calls)
#   See: RISC-V Instruction Sheet (https://github.com/TheThirdOne/rars/wiki/Supported-Instructions)
#
##################################################################

### Data Declaration Section ###

.data                               
HW:     .asciz "Hello World\n"     #define label HW as our hello world string

### Executable Code Section ###

.text

main:
    # print HW
    li      a7, 4                  # magic code to print string
    la      a0, HW                 # load address of string HW into a0
    ecall                         # HW now printed

    # exit program
    li a7, 10                      # set system call code to "terminate program"
    ecall                         # terminate program

##################################################################
#
#   NOTE:
#       Applications assembled and executed by RARS, or applications which terminate at EOF,
#       don't need the termination call. This termination call is therefore unnessecary.
#
##################################################################
