void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n, k;
  rd(n, k);
  auto x = new long[](n), y = new long[](n);
  foreach (i; 0 .. n)
    rd(x[i], y[i]);

  long mn = 9 * 10L ^^ 18;
  foreach (a; 0 .. n) {
    foreach (b; 0 .. n) {
      foreach (c; 0 .. n) {
        foreach (d; 0 .. n) {
          auto left = x[a], right = x[b], top = y[c], bottom = y[d];
          int count = 0;
          foreach (i; 0 .. n) {
            if (left <= x[i] && x[i] <= right && bottom <= y[i] && y[i] <= top) {
              count++;
            }
          }
          if (count >= k) {
            mn = min(mn, (right - left) * (top - bottom));
          }
        }
      }
    }
  }
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
