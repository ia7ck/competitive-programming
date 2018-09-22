void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n, m, a, b;
  rd(n, m, a, b);
  auto l = new int[](m), r = new int[](m);
  foreach (i; 0 .. m)
    rd(l[i], r[i]);

  int sum = 0;
  foreach (i; 0 .. n) {
    bool found = false;
    foreach (j; 0 .. m) {
      if (l[j] <= i + 1 && i + 1 <= r[j]) {
        found = true;
      }
    }
    if (found)
      sum += a;
    else
      sum += b;
  }
  writeln(sum);
}

void rd(T...)(ref T x) {
  import std.stdio, std.string, std.conv;

  auto l = readln.split;
  assert(l.length == x.length);
  foreach (i, ref e; x)
    e = l[i].to!(typeof(e));
}
