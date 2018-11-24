void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n;
  rd(n);
  auto a = readln.split.to!(int[]);

  int s = 0;
  foreach (i; 0 .. n) {
    a[i] *= n;
    s += a[i];
  }
  auto ave = s / n;
  int best_idx = 0;
  import std.math : abs;

  foreach (int i; 1 .. n) {
    if (abs(ave - a[best_idx]) > abs(ave - a[i])) {
      best_idx = i;
    }
  }
  writeln(best_idx);
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
