void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n, m;
  long k;
  rd(n, m, k);
  auto a = readln.split.to!(long[]);

  for (auto i = n - 1, r = k; i >= 0; i--) {
    if ((r -= a[i]) < 0) {
      if ((--m) == 0) {
        writeln(n - i - 1);
        return;
      }
      r = k - a[i];
    }
  }

  writeln(n);
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
