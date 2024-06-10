#include <stdio.h>
#include <stdint.h>

struct registers {
  unsigned long rbx;
  unsigned long rbp;
  unsigned long r12;
  unsigned long r13;
  unsigned long r14;
  unsigned long r15;
  unsigned long rsp;
  unsigned long rdx;
};

uint64_t set_context(struct registers *regs);
void switch_context(struct registers *regs) __attribute__((noreturn));

int main(void) {
  struct registers regs = {
    .rbx = 0,
    .rbp = 0,
    .r12 = 0,
    .r13 = 0,
    .r14 = 0,
    .r15 = 0,
    .rsp = 0,
    .rdx = 0
  };

  uint64_t ret = set_context(&regs);
  printf("%ld", ret);

  return 0;
}
