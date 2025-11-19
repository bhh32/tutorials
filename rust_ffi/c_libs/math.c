#include "math.h"

int sum(int a, int b) {
  return a + b;
}

int diff(int a, int b) {
  return a - b;
}

int prod(int a, int b) {
  return a * b;
}

int quot(int a, int b) {
  if (b != 0) {
    return a / b;
  } else {
    return 1234567890;
  }
}
