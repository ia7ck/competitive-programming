void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  import std.numeric;

  int n;
  rd(n);
  long l = 1;
  long lcm(long a, long b) {
    return a / gcd(a, b) * b;
  }

  while (n--) {
    long t;
    rd(t);
    l = lcm(l, t);
  }
  writeln(l);
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
