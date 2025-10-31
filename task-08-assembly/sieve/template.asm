##################################################################
#
#   Template for subassignment "Sieve of Eratosthenes"
#
#   Author: Viola Söderlund <violaso@kth.se>
#
#   Created: 2020-10-25
#   Translated from MIPS to RISC-V 2024-10-31 by Benjamin Widman <bwidman@kth.se>
#
#   See: RISC-V Syscall Sheet (https://github.com/TheThirdOne/rars/wiki/Environment-Calls)
#   See: RISC-V Instruction Sheet (https://github.com/TheThirdOne/rars/wiki/Supported-Instructions)
#   See: Sieve of Eratosthenes (https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes)
#
##################################################################

### Data Declaration Section ###

.data

primes:		.space  1000            # reserves a block of 1000 bytes in application memory
err_msg:	.asciz "Invalid input! Expected integer n, where 1 < n < 1001.\n"

### Executable Code Section ###

.text

main:
    # get input
    li      a7,5                   # set system call code to "read integer"
    ecall                         # read integer from standard input stream to a0

    # validate input
    li 	    t0,1001                # t0 = 1001
    slt	    t1,a0,t0		        # t1 = input < 1001
    beq     t1,zero,invalid_input # if !(input < 1001), jump to invalid_input

    li	    t0,1                   # t0 = 1
    slt     t1,t0,a0		        # t1 = 1 < input
    beq     t1,zero,invalid_input # if !(1 < input), jump to invalid_input
    
    # initialise primes array
    la	    t0,primes              # s1 = address of the first element in the array
    li 	    t1,999
    li 	    t2,0
    li	    t3,1
init_loop:
    sb	    t3, (t0)              # primes[i] = 1
    addi    t0, t0, 1             # increment pointer
    addi    t2, t2, 1             # increment counter
    bne	    t2, t1, init_loop     # loop if counter != 999
    
    ### Continue implementation of Sieve of Eratosthenes ###

    ### Print nicely to output stream ###
    
    # exit program
    j       exit_program

invalid_input:
    # print error message
    li      a7, 4                  # set system call code "print string"
    la      a0, err_msg            # load address of string err_msg into the system call argument registry
    ecall                         # print the message to standard output stream

exit_program:
    # exit program
    li a7, 10                      # set system call code to "terminate program"
    ecall                         # exit program
