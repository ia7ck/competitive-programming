void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n, k;
  rd(n, k);
  auto a = readln.split.to!(int[]);

  auto tree = new SegmentTree(a);
  long s = 0;
  for (int i = 1; i <= n; i++)
    s += tree._min(max(0, i - k), i);
  writeln(s);
}

class SegmentTree {
  import std.algorithm;

  int n = 1;
  const int inf = 10 ^^ 9 + 1;
  int[] dat;
  this(int[] a) {
    int N = cast(int) a.length;
    while (n < N)
      n *= 2;
    dat.length = n * 2 - 1;
    fill(dat, inf);
    foreach (i; 0 .. N)
      dat[i + n - 1] = a[i];
    for (int i = n - 2; i >= 0; i--)
      dat[i] = min(dat[i * 2 + 1], dat[i * 2 + 2]);
  }

  int _min(int ql, int qr) {
    return _min(ql, qr, 0, 0, n);
  }

  int _min(int ql, int qr, int i, int il, int ir) {
    if (qr <= il || ir <= ql)
      return inf;
    if (ql <= il && ir <= qr)
      return dat[i];
    int m = (il + ir) / 2;
    return min(_min(ql, qr, i * 2 + 1, il, m), _min(ql, qr, i * 2 + 2, m, ir));
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
