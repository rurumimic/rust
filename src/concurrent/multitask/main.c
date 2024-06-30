#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

struct registers {
  uint64_t rbx;
  uint64_t rbp;
  uint64_t r12;
  uint64_t r13;
  uint64_t r14;
  uint64_t r15;
  uint64_t rsp;
  uint64_t rdx;
};

uint64_t set_context(struct registers *regs);
void switch_context(struct registers *regs) __attribute__((noreturn));

int main(void) {
  struct registers *regs = malloc(sizeof(struct registers));
  memset(regs, 0, sizeof(struct registers));

  uint64_t ret = set_context(regs);
  printf("%lu\n", ret);

  free(regs);
  return 0;
}

