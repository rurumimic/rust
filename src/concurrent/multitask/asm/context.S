#define SET_CONTEXT set_context
#define SWITCH_CONTEXT switch_context
#define C_SET_CONTEXT c_set_context

// export symbols
.global SET_CONTEXT
.global SWITCH_CONTEXT
.global C_SET_CONTEXT

.text // code section
.align 4

SET_CONTEXT:
  // | return address | <- rsp
  // |                |
  // |                |

  pop %rbp // pop return address (after set_context()) and save it in rbp
  xor %eax, %eax // set eax to 0. return value: 0

  // | return address |    rbp = 56(%rdi)
  // |                | <- rsp
  // |                |    rsp + 8 = 48(%rdi)

  movq %rbx, (%rdi) // save rbx to the memory pointed by rdi (**ctx)
  movq %rbp, 8(%rdi) // save rbp (return address) to rdi + 8
  movq %r12, 16(%rdi)
  movq %r13, 24(%rdi)
  movq %r14, 32(%rdi)
  movq %r15, 40(%rdi)

  // save current stack pointer address
  // Have to add 8 to %rsp to run it, but I don't know why.
  // Probably because of push %rsi in SWITCH_CONTEXT.
  lea 8(%rsp), %rdx // load rdx with rsp + 8
  movq %rdx, 48(%rdi) // save rdx to rdi + 48

  // save the return address (after the call to set_context)
  push %rbp // push rbp to the stack
  movq (%rsp), %rdx // load rdx with the value pointed by rsp (return address)
  movq %rdx, 56(%rdi) // save rdx to rdi + 56

  // | return address | <- rsp = rbp = 56(%rdi)
  // |                |
  // |                |    48(%rdi)

  ret // return to the caller (eax is 0)

.text
.align 4

SWITCH_CONTEXT:
  // A stack frame
  // | return address | <- rsp
  // |                | 

  xor %eax, %eax // set eax to 0
  inc %eax // set eax to 1. return value: 1

  pop %rsi // pop return address (after set_context) and save it in rsi

  // A stack frame
  // | return address |    rsi
  // |                | <- rsp = 48(%rdi)

  movq (%rdi), %rbx // load rbx with the value pointed by rdi
  movq 8(%rdi), %rbp // load rbp with the value pointed by rdi + 8
  movq 16(%rdi), %r12
  movq 24(%rdi), %r13
  movq 32(%rdi), %r14
  movq 40(%rdi), %r15

  // point the stack pointer to the stack memory allocated by Context::new()
  movq 48(%rdi), %rdx
  movq %rdx, %rsp

  // B stack frame
  // ================== allocated memory area
  // |                |
  // |                |
  // |                |
  // ================== allocated memory area
  // |                | <- rsp = 48(%rdi)

  // Even without these two lines, it runs fine.
  // addq $0x8, %rsp // It seems to be going out of the allocated memory area.
  // push %rbp       // I don't know what role %rbp plays in 64bit.

  push %rsi // save return address (after set_context) in the stack

  // B stack frame
  // ================== allocated memory area
  // |                |
  // |                |
  // | return address | <- rsp = rsi
  // ================== allocated memory area
  // |                |

  movq 56(%rdi), %rdx // load entry_point() address
  jmpq *%rdx // jump to entry_point()

.text // code section
.align 4

C_SET_CONTEXT:
  // | old rbp        | <- rsp = rbp
  // | return address |    rsp + 8  = 8(%rdi)
  // |                |    rsp + 16
  // |                |    rsp + 24 = 48(%rdi)

  // prologue
  pushq %rbp
  movq %rsp, %rbp

  xor %rax, %rax // set rax to 0.
  inc %rax // set rax to 1.
  inc %rax // set rax to 2. return value: 2

  movq %rbx, (%rdi) // save rbx to the memory pointed by rdi (**ctx)
  movq %rbp, 8(%rdi) // save rbp (return address) to rdi + 8
  movq %r12, 16(%rdi)
  movq %r13, 24(%rdi)
  movq %r14, 32(%rdi)
  movq %r15, 40(%rdi)

  // save current stack pointer address
  lea 24(%rsp), %rdx // load rdx with rsp + 8
  movq %rdx, 48(%rdi) // save rdx to rdi + 48

  // save the return address (after the call to set_context)
  movq 8(%rsp), %rdx // load return address
  movq %rdx, 56(%rdi) // save rdx to rdi + 56

  // epilogue
  leave // restore the stack frame
  retq // return to the caller (rax is 2)
