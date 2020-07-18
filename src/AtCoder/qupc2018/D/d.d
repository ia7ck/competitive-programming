void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n, m, l;
  rd(n, m, l);
  auto ts = readln.split.to!(int[]);
  auto xs = new int[](m), as = new int[](m);
  foreach (i; 0 .. m)
    rd(xs[i], as[i]);
  auto yb = new int[][](n);
  foreach (_; 0 .. l) {
    int y, b;
    rd(y, b);
    yb[y - 1] ~= b;
  }
  foreach (i; 0 .. n)
    sort(yb[i]);

  struct T {
    int l, r;
  }

  import std.range;

  T[] seq;
  foreach (i; 0 .. m) {
    auto t1 = as[i] + ts[xs[i] - 1];
    auto ub = yb[xs[i] - 1].assumeSorted.upperBound(t1);
    if (ub.length > 0) {
      seq ~= T(as[i], ub[0] + ts[xs[i] - 1]);
    }
  }
  seq.sort!"a.r==b.r ? a.l>b.l : a.r<b.r";
  int k = 0, last = 0;
  foreach (s; seq) {
    if (last < s.l) {
      last = s.r;
      k += 2;
    }
  }
  if (as.any!((a) => (a > last))) {
    k++;
  }
  writeln(k);
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
