void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int q;
  rd(q);
  while (q--) {
    int n, k;
    rd(n, k);
    auto a = readln.split.to!(int[]);
    a.sort;
    auto lb = a[n - 1] - k, ub = a[0] + k;
    if (lb <= ub) {
      writeln(ub);
    } else {
      writeln(-1);
    }
  }
}

// [a[n - 1] - k, a[0] + k]

void rd(T...)(ref T x) {
  import std.stdio : readln;
  import std.string : split;
  import std.conv : to;

  auto l = readln.split;
  assert(l.length == x.length);
  foreach (i, ref e; x)
    e = l[i].to!(typeof(e));
}
