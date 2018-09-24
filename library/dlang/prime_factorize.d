void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  foreach (x; [2, 3, 4, 5, 6, 7, 8, 9, 10, 10 ^^ 9 + 7]) {
    auto pf = prime_fact(x);
    assert(x == reduce!"a*b"(1, pf));
  }
}

int[] prime_fact(int x) {
  assert(x >= 0);
  int[] ret;
  for (int i = 2; i * i <= x; i++) {
    while (x % i == 0) {
      ret ~= i;
      x /= i;
    }
  }
  if (x > 1) { // これ忘れがち
    ret ~= x;
  }
  return ret;
}
