#include "gmp.h"

typedef struct {
  mpz_t step, ident;
} flm;

typedef struct {
  mpz_t tl, tr, bl, br;
} mat;

// these variables are used in the functions as storage
// they're exterior so that I can initialize them once each.
mpz_t s0, s1, s2;
flm base;
mat mbase;

#define INITIALIZERS                                                           \
  mpz_init(base.step);                                                         \
  mpz_init(base.ident);                                                        \
  mpz_init(mbase.tl);                                                          \
  mpz_init(mbase.tr);                                                          \
  mpz_init(mbase.bl);                                                          \
  mpz_init(mbase.br);                                                          \
  mpz_init(s0);                                                                \
  mpz_init(s1);                                                                \
  mpz_init(s2);

void flm_mul(flm *rop, const flm op) {
  mpz_add(s2, rop->step, rop->ident);
  mpz_add(s0, op.step, op.ident);

  mpz_mul(s1, s0, s2);

  mpz_mul(s0, rop->ident, op.ident);
  mpz_sub(s1, s1, s0);

  mpz_addmul(s0, rop->step, op.step);

  mpz_set(rop->step, s1);
  mpz_set(rop->ident, s0);
}

void mat_mul(mat *rop, const mat op) {
  mpz_mul(s0, rop->tl, op.tl);
  mpz_addmul(s0, rop->tr, op.bl);

  mpz_mul(rop->tr, rop->tr, op.br);
  mpz_addmul(rop->tr, rop->tl, op.tr);

  mpz_set(rop->tl, s0);

  mpz_mul(s0, rop->bl, op.tl);
  mpz_addmul(s0, rop->br, op.bl);

  mpz_mul(rop->br, rop->br, op.br);
  mpz_addmul(rop->br, rop->bl, op.tr);

  mpz_set(rop->bl, s0);
}

void flm_pow(flm *ret, unsigned long idx) {
  mpz_set(base.step, ret->step);
  mpz_set(base.ident, ret->ident);
  idx--;
  while (idx) {
    if (idx % 2)
      flm_mul(ret, base);
    idx /= 2;
    if (idx)
      flm_mul(&base, base);
  }
}

void mat_pow(mat *ret, unsigned long idx) {
  mpz_set(mbase.tl, ret->tl);
  mpz_set(mbase.tr, ret->tr);
  mpz_set(mbase.bl, ret->bl);
  mpz_set(mbase.br, ret->br);
  idx--;
  while (idx) {
    if (idx % 2)
      mat_mul(ret, mbase);
    idx /= 2;
    if (idx)
      mat_mul(&mbase, mbase);
  }
}

#include <stdio.h>

// 40 is 0.5, 41 is 0.8, 42 is 1.3
// recursive solution
unsigned long rec_fib(unsigned n) {
  switch (n) {
  case 0:
    return 0;
  case 1:
    return 1;
  default:
    return rec_fib(n - 1) + rec_fib(n - 2);
  }
}

// 72, 74
unsigned long gr_fib(unsigned long n) {
  switch (n) {
  case 0:
    return 0;
  case 1:
    return 1;
  case 2:
    return 1;
  case 10:
    return 55;
  case 20:
    return 6765;
  default:
    return gr_fib(n - 3) + 2 * gr_fib(n - 2);
  }
}

// iterative fibonacci
// 600k, 800k
int iter_fib(mpz_t rop0, mpz_t rop1, unsigned long n) {
  mpz_set_ui(rop0, 0);
  mpz_set_ui(rop1, 1);
  int bigger = 1;
  while (n--) {
    mpz_add(bigger ? rop0 : rop1, bigger ? rop0 : rop1, bigger ? rop1 : rop0);
    bigger = 1 - bigger;
  }
  return bigger;
}

// matrix exponentiation
// 8M, 13M
void fm_fib(mpz_t rop, unsigned long n) {
  mat step;
  mpz_init(step.tl);
  mpz_init(step.tr);
  mpz_init(step.bl);
  mpz_init(step.br);
  mpz_set_ui(step.tl, 1);
  mpz_set_ui(step.tr, 1);
  mpz_set_ui(step.bl, 1);

  mat_pow(&step, n);

  mpz_set(rop, step.tr);
}

// compact matrix exponentiation
// 70M, 128M
void cm_fib(mpz_t rop, unsigned long n) {
  flm step;
  mpz_init(step.step);
  mpz_init(step.ident);
  mpz_set_ui(step.step, 1);

  flm_pow(&step, n);

  mpz_set(rop, step.step);
}

int main(void) {
  INITIALIZERS;
  mpz_t x;
  mpz_init(x);

  cm_fib(x, 1 << 27);

  gmp_printf("%Zd\n", x);
}
