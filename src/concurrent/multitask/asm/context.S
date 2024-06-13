#define SET_CONTEXT set_context
#define SWITCH_CONTEXT switch_context

// export symbols
.global SET_CONTEXT
.global SWITCH_CONTEXT

.text // code section
.align 4 // align to 16 bytes

SET_CONTEXT:
  pop %rbp // pop return address and save it in rbp
  xor %eax, %eax // set eax to 0. return value: 0

  movq %rbx, (%rdi) // save rbx to the memory pointed by rdi
  movq %rbp, 8(%rdi) // save rbp to rdi + 8
  movq %r12, 16(%rdi)
  movq %r13, 24(%rdi)
  movq %r14, 32(%rdi)
  movq %r15, 40(%rdi)

  // save current stack pointer address
  lea 8(%rsp), %rdx // load rdx with rsp + 8
  movq %rdx, 48(%rdi) // save rdx to rdi + 48

  // save the return address
  push %rbp // push rbp to the stack
  movq (%rsp), %rdx // load rdx with the value pointed by rsp (return address)
  movq %rdx, 56(%rdi) // save rdx to rdi + 56

  ret // return to the caller (eax is 0)

.text
.align 4

SWITCH_CONTEXT:
  xor %eax, %eax // set eax to 0
  inc %eax // set eax to 1. return value: 1

  pop %rsi // pop value from the stack and save it in rsi

  movq (%rdi), %rbx // load rbx with the value pointed by rdi
  movq 8(%rdi), %rbp // load rbp with the value pointed by rdi + 8
  movq 16(%rdi), %r12
  movq 24(%rdi), %r13
  movq 32(%rdi), %r14
  movq 40(%rdi), %r15

  movq 48(%rdi), %rdx
  movq %rdx, %rsp // load rsp with the value pointed by rdx. stack pointer
  addq $0x8, %rsp // rsp = rsp + 8. skip the return address

  push %rbp // push rbp to the stack
  push %rsi // push rsi to the stack

  movq 56(%rdi), %rdx // load rdx with the value pointed by rdi + 56. return address
  jmpq *%rdx // jump to the address pointed by rdx

