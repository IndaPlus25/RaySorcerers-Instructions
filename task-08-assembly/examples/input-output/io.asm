##################################################################
#
#   RISC-V assembly code example
#   - I/O
#
#   Author: Tobias Hansson <tohans@kth.se>, Viola Söderlund <violaso@kth.se>
#
#   Last updated: 2020-10-24
#   Translated from MIPS to RISC-V 2024-10-31 by Benjamin Widman <bwidman@kth.se>
#
#   See: RISC-V Syscall Sheet (https://github.com/TheThirdOne/rars/wiki/Environment-Calls)
#   See: RISC-V Instruction Sheet (https://github.com/TheThirdOne/rars/wiki/Supported-Instructions)
#
##################################################################

main: 
    # get input
    li  a7, 5                         # set system call code to "read integer"
    ecall                             # read integer from standard input stream to a0

    # calculate output
    mul a0, a0, a0                   # a0 = a0 * a0

    # print output
    li  a7, 1                          # set system call code to "print integer"
    ecall                             # print square of input integer to output stream

##################################################################
#
#   NOTE:
#       The Executable Code Section is the default section. Therefore ".text" isn't needed.
#
##################################################################
