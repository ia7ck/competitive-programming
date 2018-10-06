void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n;
  rd(n);
  auto x = new int[](n), y = new int[](n), h = new long[](n);
  foreach (i; 0 .. n)
    rd(x[i], y[i], h[i]);

  long min_h = 10 ^^ 9, max_h = 0;
  foreach (i; 0 .. n) {
    min_h = min(min_h, h[i]);
    max_h = max(max_h, h[i]);
  }
  min_h = max(1, min_h - 200);
  max_h = max_h + 200;
  import std.math : abs;

  for (int cx = 0; cx <= 100; cx++) {
    for (int cy = 0; cy <= 100; cy++) {
      for (long height = min_h; height <= max_h; height++) {
        foreach (i; 0 .. n) {
          if (max(0, height - abs(x[i] - cx) - abs(y[i] - cy)) != h[i]) {
            goto hell;
          }
        }
        writeln(cx, " ", cy, " ", height);
        return;
      hell:;

      }
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
