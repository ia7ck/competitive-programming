void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int a;
  rd(a);

  int digit_sum(int x) {
    int ret = 0;
    while (x) {
      ret += x % 10;
      x /= 10;
    }
    return ret;
  }

  for (int n = a;; n++) {
    if (digit_sum(n) % 4 == 0) {
      writeln(n);
      break;
    }
  }

}

void rd(T...)(ref T x) {
  import std.stdio : readln;
  import std.string : split;
  import std.conv : to;

  auto l = readln.split;
  assert(l.length == x.length);
  foreach (i, ref e; x)
    e = l[i].to!(typeof(e));
}
