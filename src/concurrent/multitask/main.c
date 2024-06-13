#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// struct registers {
//   uint64_t rbx;
//   uint64_t rbp;
//   uint64_t r12;
//   uint64_t r13;
//   uint64_t r14;
//   uint64_t r15;
//   uint64_t rsp;
//   uint64_t rdx;
// };

struct structure {
  char a;
};

// uint64_t set_context(struct registers *regs);
// void switch_context(struct registers *regs) __attribute__((noreturn));

void nothing() { return; }
void printer(struct structure *s, int a) { printf("%c %d", s->a, a); }

int main(void) {
  struct structure *s = malloc(sizeof(struct structure));
  s->a = 'A';
  int a = 17;

  int i = 0;
  for (i = 0; i < 3; i++) {
    nothing();
  }

  printer(s, a);
  free(s);

  // struct registers *regs = malloc(sizeof(struct registers));
  // memset(regs, 3, sizeof(struct registers));

  // uint64_t ret = set_context(regs);
  // printf("%ld", ret);

  // free(regs);
  return 0;
}
