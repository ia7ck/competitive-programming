void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n, m, k;
  rd(n, m, k);
  import std.array;

  auto r = readln.split
    .to!(int[])
    .map!((e) => (e - 1))
    .array;
  auto d = new int[][](n, n);
  const int inf = 10 ^^ 9;
  foreach (i; 0 .. n)
    foreach (j; 0 .. n)
      d[i][j] = inf;
  foreach (i; 0 .. n)
    d[i][i] = 0;
  foreach (_; 0 .. m) {
    int a, b, c;
    rd(a, b, c);
    a--;
    b--;
    d[a][b] = d[b][a] = c;
  }
  foreach (_; 0 .. n)
    foreach (i; 0 .. n)
      foreach (j; 0 .. n)
        d[i][j] = min(d[i][j], d[i][_] + d[_][j]);
  import std.range : iota;

  auto idxs = iota(0, k).array;
  int mn = inf;
  do {
    int s = 0;
    foreach (i; 1 .. k) {
      s += d[r[idxs[i - 1]]][r[idxs[i]]];
    }
    mn = min(mn, s);
  }
  while (nextPermutation(idxs));
  writeln(mn);
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
