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

uint64_t c_set_context(struct registers *regs);

int main(void) {
  uint64_t result;
  struct registers *regs = (struct registers *)malloc(sizeof(struct registers));
  memset(regs, 0, sizeof(struct registers));

  result = c_set_context(regs);
  printf("Result: %lu\n", result);

  free(regs);
  return 0;
}
