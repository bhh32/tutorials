#include "math_cpp.h"

int Math::add(int a, int b) {
  return a + b;
}

int Math::sub(int a, int b) {
  return a - b;
}

int Math::mul(int a, int b) {
  return a * b;
}

int Math::div(int a, int b) {
  if(b != 0) {
    return a / b;
  }

  return 1234567890;
}
