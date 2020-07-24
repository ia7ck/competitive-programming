void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int h, w;
  rd(h, w);
  int n;
  rd(n);
  auto a = readln.split.to!(int[]);

  int[] b;
  foreach (i; 0 .. n) {
    foreach (_; 0 .. a[i]) {
      b ~= i + 1;
    }
  }
  auto c = new int[][](h, w);
  foreach (i; 0 .. h) {
    foreach (j; 0 .. w) {
      if ((i & 1) == 0) {
        c[i][j] = b[i * w + j];
      } else {
        c[i][w - j - 1] = b[i * w + j];
      }
    }
  }
  writefln("%(%(%s %)\n%)", c);
}

void rd(T...)(ref T x) {
  import std.stdio, std.string, std.conv;

  auto l = readln.split;
  assert(l.length == x.length);
  foreach (i, ref e; x)
    e = l[i].to!(typeof(e));
}
